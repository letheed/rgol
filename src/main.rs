#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![deny(unsafe_code)]

use std::{result::Result as StdResult, time::Duration};

use anyhow::Context;
use clap::{App, ArgMatches};
use rgol::{GridSizeError, World};

mod screen;

const TICK_MS: u64 = 50;
const GRIDS_MSG: &str = "\
GRIDS:
    Grids must be rectangular. Whitespace is ignored.
    '·' (U+00B7 MIDDLE DOT) is a dead cell. Anything else is a living cell.";

type Result<T = ()> = anyhow::Result<T>;

fn main() -> Result {
    match app().get_matches().subcommand() {
        Some(("grid", args)) => grid_subcommand(args),
        Some(("play", args)) => play_subcommand(args),
        _ => unreachable!("SubcommandRequiredElseHelp prevents `None`"),
    }
}

#[allow(deprecated)]
fn app() -> App<'static> {
    use clap::{clap_app, crate_authors, crate_description, crate_version};

    fn is_number(s: &str) -> StdResult<(), String> {
        if s.chars().all(|c| c.is_digit(10)) {
            Ok(())
        } else {
            Err(format!("expected a number, found \"{}\"", s))
        }
    }

    clap_app!(rgol =>
        (author: crate_authors!())
        (version: crate_version!())
        (about: crate_description!())
        (after_help: GRIDS_MSG)
        (@setting SubcommandRequiredElseHelp)
        (@subcommand grid =>
            (about: "Prints an empty grid")
            (@arg NROW: {is_number} * "Number of rows")
            (@arg NCOL: {is_number} * "Number of columns")
            (@arg space: -s --space "Adds spaces to the grid"))
        (@subcommand play =>
            (about: "Plays the game (CTRL-c to exit)")
            (@arg FILE: * "File containing the grid")
            (@arg TICK_MS: {is_number} "Elapsed time between iterations in ms")
        )
    )
}

#[test]
fn verify_app() {
    app().debug_assert();
}

/// Prints an empty grid.
fn grid_subcommand(args: &ArgMatches) -> Result {
    let nrow: usize = args.value_of_t("NROW")?;
    let ncol: usize = args.value_of_t("NCOL")?;
    if nrow == 0 || ncol == 0 {
        return Err(GridSizeError::Zero.into());
    }
    let line = if args.is_present("space") {
        let mut line = "· ".repeat(ncol);
        line.pop();
        line
    } else {
        "·".repeat(ncol)
    };
    for _ in 0..nrow {
        println!("{line}");
    }
    Ok(())
}

/// Loads a grid from a file and plays it.
fn play_subcommand(args: &ArgMatches) -> Result {
    use clap::ErrorKind;

    let filename = args.value_of("FILE").expect("FILE is required");
    let tick_ms = match args.value_of_t("TICK_MS") {
        Err(error) if error.kind() == ErrorKind::ArgumentNotFound => Ok(TICK_MS),
        result => result,
    }?;
    let tick = Duration::from_millis(tick_ms);
    let world = { || -> Result<_> { Ok(std::fs::read_to_string(filename)?.parse()?) } }()
        .with_context(|| format!("in {}", filename))?;
    play_world(world, tick)
}

/// Plays a world and prints every generation to the terminal.

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
        println!("{world}");
        world.tick();
        deadline += tick;
        if sigtrap.wait(deadline).is_some() {
            return Ok(());
        }
    }
}

/// Plays a world and prints every generation to the terminal.

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
            println!("{world}");
            world.tick();
            deadline += tick;
            let duration = deadline.saturating_duration_since(Instant::now());
            // TODO: Use `Receiver::recv_deadline` when it becomes stable.
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
