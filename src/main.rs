#[macro_use]
extern crate clap;
extern crate libc;
extern crate signal;

use clap::ArgMatches;
use std::time::Duration;
use world::World;

mod screen;
mod world;

const TICK_MS: u64 = 50;

static MAPS_MSG: &str = "\
MAPS:
    Maps must be rectangular. Whitespace is ignored.
    '·' is a dead cell. Anything else is a living cell.
";

fn main() {
    fn is_number(s: String) -> Result<(), String> {
        if s.chars().all(|c| c.is_digit(10)) {
            Ok(())
        } else {
            Err(format!("expected a number, found \"{}\"", s))
        }
    }

    let matches = clap_app!( rgol =>
        (about: "Conway's game of life for terminal in Rust")
        (author: crate_authors!())
        (version: crate_version!())
        (after_help: MAPS_MSG)
        (@setting SubcommandRequiredElseHelp)
        (@setting VersionlessSubcommands)
        (@subcommand genmap =>
            (about: "Prints an empty map")
            (after_help: MAPS_MSG)
            (@arg NROW: {is_number} * "Number of rows")
            (@arg NCOL: {is_number} * "Number of columns")
            (@arg space: -s --space "Adds spaces to the map"))
        (@subcommand play =>
            (about: "Plays the game (CTRL-c to exit)")
            (after_help: MAPS_MSG)
            (@arg FILE: * "File containing the map")
            (@arg TICK_MS: {is_number} "Elapsed time between iterations in ms")
        )
    ).get_matches();

    match matches.subcommand() {
        ("genmap", Some(args)) => genmap(args),
        ("play", Some(args)) => play(args),
        _ => unreachable!(),
    }
}

fn genmap(args: &ArgMatches) {
    let nrow = args.value_of("NROW").expect("NROW has no value").parse().expect("NROW is not a number");
    let ncol = args.value_of("NCOL").expect("NCOL has no value").parse().expect("NCOL is not a number");
    if nrow == 0 || ncol == 0 {
        return;
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
}

fn play(args: &ArgMatches) {
    let filename = args.value_of("FILE").expect("FILE has no value");
    let tick_ms = args.value_of("TICK_MS").map_or(TICK_MS, |tms| tms.parse().expect("TICK_MS is not a number"));
    let tick = Duration::from_millis(tick_ms);
    match World::load(filename) {
        Ok(world) => play_world(world, tick),
        Err(err) => eprintln!("error: {}", err),
    }
}

fn play_world(mut world: World, tick: Duration) {
    use screen::Screen;
    use std::time::Instant;
    use signal::Signal;
    use signal::trap::Trap;

    let sigtrap = Trap::trap(&[Signal::SIGINT]);
    let screen = Screen::new();
    let mut deadline = Instant::now();
    loop {
        screen.clear();
        println!("{}", world);
        world.next();
        deadline += tick;
        if sigtrap.wait(deadline).is_some() {
            break;
        }
    }
}
