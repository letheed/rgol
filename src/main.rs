#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![deny(unsafe_code)]

use std::{
    fmt::{self, Display},
    result::Result as StdResult,
    time::Duration,
};

use anyhow::{anyhow, Context, Error};
use clap::ArgMatches;
use rgol::World;

mod screen;

const TICK_MS: u64 = 50;
const MAPS_MSG: &str = "\
MAPS:
    Maps must be rectangular. Whitespace is ignored.
    '·' (U+00B7 MIDDLE DOT) is a dead cell. Anything else is a living cell.";

type Result<T = ()> = anyhow::Result<T>;

fn main() {
    if let Err(err) = run() {
        eprintln!("error{}", DisplayCauses(err));
        std::process::exit(1);
    }
}

fn run() -> Result {
    use clap::{clap_app, crate_authors, crate_version};

    #[allow(clippy::needless_pass_by_value)]
    fn is_number(s: String) -> StdResult<(), String> {
        if s.chars().all(|c| c.is_digit(10)) {
            Ok(())
        } else {
            Err(format!("expected a number, found \"{}\"", s))
        }
    }

    let matches = clap_app!(rgol =>
        (about: "Conway's game of life for terminal in Rust")
        (author: crate_authors!())
        (version: crate_version!())
        (after_help: MAPS_MSG)
        (@setting ColoredHelp)
        (@setting SubcommandRequiredElseHelp)
        (@setting VersionlessSubcommands)
        (@subcommand genmap =>
            (about: "Prints an empty map")
            (@setting ColoredHelp)
            (@arg NROW: {is_number} * "Number of rows")
            (@arg NCOL: {is_number} * "Number of columns")
            (@arg space: -s --space "Adds spaces to the map"))
        (@subcommand play =>
            (about: "Plays the game (CTRL-c to exit)")
            (@setting ColoredHelp)
            (@arg FILE: * "File containing the map")
            (@arg TICK_MS: {is_number} "Elapsed time between iterations in ms")
        )
    )
    .get_matches();

    match matches.subcommand() {
        ("genmap", Some(args)) => genmap(args),
        ("play", Some(args)) => play(args),
        _ => anyhow::bail!("subcommands failed to match properly"),
    }
}

/// "genmap" subcommand.
///
/// Prints an empty map.
fn genmap(args: &ArgMatches<'_>) -> Result {
    let nrow = args
        .value_of("NROW")
        .ok_or_else(|| anyhow!("NROW has no value"))?
        .parse()
        .context("NROW is not a number")?;
    let ncol = args
        .value_of("NCOL")
        .ok_or_else(|| anyhow!("NCOL has no value"))?
        .parse()
        .context("NCOL is not a number")?;
    if nrow == 0 || ncol == 0 {
        return Ok(());
    }
    let line = if args.is_present("space") {
        let mut line = "· ".repeat(ncol);
        line.pop();
        line
    } else {
        "·".repeat(ncol)
    };
    for _ in 0..nrow {
        println!("{}", line);
    }
    Ok(())
}

/// "play" subcommand.
///
/// Loads a map from a file and seeds the world with it, then plays it.
fn play(args: &ArgMatches<'_>) -> Result {
    let filename = args.value_of("FILE").ok_or_else(|| anyhow!("FILE has no value"))?;
    let tick_ms = args
        .value_of("TICK_MS")
        .map_or(Some(TICK_MS), |t_ms| t_ms.parse().ok())
        .ok_or_else(|| anyhow!("TICK_MS is not a number"))?;
    let tick = Duration::from_millis(tick_ms);
    let world = load_world(filename).with_context(|| filename.to_string())?;
    play_world(world, tick)
}

/// Loads and parses the world from a file.
fn load_world(filename: &str) -> Result<World> {
    Ok(std::fs::read_to_string(filename)?.parse()?)
}

/// Plays the world.
///
/// Prints every generation to the terminal screen.
#[cfg(target_os = "linux")]
// The `signal` crate only defines `Trap::wait` for linux, though it should
// work anywhere `sigtimedwait` is defined (which doesn’t include macOS).
fn play_world(mut world: World, tick: Duration) -> Result {
    use std::time::Instant;

    use screen::Screen;
    use signal::{trap::Trap, Signal};

    let sigtrap = Trap::trap(&[Signal::SIGINT]);
    let screen = Screen::init()?;
    let mut deadline = Instant::now();
    loop {
        screen.clear();
        println!("{}", world);
        world.next();
        deadline += tick;
        if sigtrap.wait(deadline).is_some() {
            return Ok(());
        }
    }
}

/// Plays the world.
///
/// Prints every generation to the terminal screen.
#[cfg(all(unix, not(target_os = "linux")))]
// Anywhere `Trap::wait` isn’t defined we’ll have to rely on `sigwait`
// (through `Iterator::next`) though presumably some of those targets
// do define `sigtimedwait`.
fn play_world(mut world: World, tick: Duration) -> Result {
    use std::{
        panic,
        sync::mpsc::{channel, RecvTimeoutError},
        thread,
        time::Instant,
    };

    use screen::Screen;
    use signal::{trap::Trap, Signal};

    let sigtrap = Trap::trap(&[Signal::SIGINT]);
    let screen = Screen::init()?;
    let mut deadline = Instant::now();
    let (sender, receiver) = channel();
    let player = thread::spawn(move || {
        loop {
            screen.clear();
            println!("{}", world);
            world.next();
            deadline += tick;
            let duration = deadline.saturating_duration_since(Instant::now());
            // TODO Use `Receiver::recv_deadline` when it becomes stable.
            match receiver.recv_timeout(duration) {
                Err(RecvTimeoutError::Timeout) => {}
                Ok(()) | Err(RecvTimeoutError::Disconnected) => return,
            }
        }
    });
    let signal = sigtrap.into_iter().next();
    sender.send(())?;
    if let Err(e) = player.join() {
        panic::resume_unwind(e);
    }
    match signal {
        Some(Signal::SIGINT) => Ok(()),
        Some(_) => anyhow::bail!("`Trap` returned with unexpected {:?} signal", signal),
        None => anyhow::bail!("`Trap` returned but no signal was received"),
    }
}

/// Displays the causes of an `Error` recursively.
struct DisplayCauses(Error);

impl Display for DisplayCauses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cause in self.0.chain() {
            write!(f, ": {}", cause)?;
        }
        Ok(())
    }
}
