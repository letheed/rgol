#[macro_use]
extern crate clap;
extern crate libc;
extern crate signal;

use clap::ArgMatches;
use std::time::Duration;
use world::World;

#[macro_use]
mod macros;
mod world;

const MAPS_MSG: &'static str = "\
MAPS:
    Maps must be rectangular. Whitespace is ignored.
    '·' is a dead cell. Anything else is a living cell.
";
const TICK_MS: u64 = 50;

fn main() {
    use clap::{App, AppSettings, Arg, SubCommand};

    fn is_number(s: String) -> Result<(), String> {
        if s.chars().all(|c| c.is_digit(10)) { Ok(()) }
        else { Err(format!("in argument \"{}\": expected a number", s)) }
    }

    let app = App::new("rgol")
        .about("Conway's game of life for terminal in Rust")
        .author(crate_authors!())
        .version(crate_version!())
        .after_help(MAPS_MSG)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(SubCommand::with_name("genmap")
                    .about("Prints an empty map")
                    .after_help(MAPS_MSG)
                    .arg(Arg::with_name("NROW")
                         .help("Number of rows")
                         .required(true)
                         .validator(is_number))
                    .arg(Arg::with_name("NCOL")
                         .help("Number of columns")
                         .required(true)
                         .validator(is_number))
                    .arg(Arg::with_name("space")
                         .help("Adds spaces to the map")
                         .short("s")
                         .long("space")))
        .subcommand(SubCommand::with_name("play")
                    .about("Plays the game (CTRL-c to exit)")
                    .after_help(MAPS_MSG)
                    .arg(Arg::with_name("FILE")
                         .help("File containing the map")
                         .required(true))
                    .arg(Arg::with_name("TICK_MS")
                         .help("Elapsed time between iterations in ms")
                         .validator(is_number)))
        .get_matches();
    match app.subcommand() {
        ("genmap", Some(args)) => genmap(args),
        ("play",   Some(args)) => play(args),
        _                      => unreachable!(),
    }
}

fn genmap(args: &ArgMatches) {
    use std::io::{Write, stdout};
    use std::iter::repeat;

    let nrow = args.value_of("NROW").expect("NROW has no value").parse().expect("NROW is not a number");
    let ncol = args.value_of("NCOL").expect("NCOL has no value").parse().expect("NCOL is not a number");
    if nrow == 0 || ncol == 0 { return }
    let line = if args.is_present("space") {
        let mut line: String = repeat("· ").take(ncol).collect();
        line.pop();
        line
    }
    else { repeat('·').take(ncol).collect() };
    for _ in 0..nrow { writeln!(&mut stdout(), "{}", line).unwrap(); }
}

fn play(args: &ArgMatches) {
    let filename = args.value_of("FILE").expect("FILE has no value");
    let tick_ms = args.value_of("TICK_MS").map_or(TICK_MS, |tms| tms.parse().expect("TICK_MS is not a number"));
    let tick = Duration::from_millis(tick_ms);
    match World::load(filename) {
        Ok(world) => play_world(world, tick),
        Err(err)  => eprintln!("error: {}", err),
    }
}

fn play_world(mut world: World, tick: Duration) {
    use std::time::Instant;
    use signal::trap::Trap;

    prep_term();
    let sigtrap = Trap::trap(&[libc::SIGINT]);
    let mut deadline = Instant::now();
    loop {
        clear_screen();
        println!("{}", world);
        world.next();
        deadline += tick;
        if sigtrap.wait(deadline).is_some() { break }
    }
    restore_term();
}

fn prep_term() {
    // save screen, cursor position and hide cursor
    print!("{}", concat!("\x1B[?1049h", "\x1B7", "\x1B[?25l"));
}

fn clear_screen() {
    // clear screen and move cursor to home position
    print!("{}", concat!("\x1B[2J", "\x1B[H"));
}

fn restore_term() {
    // restore screen, cursor position and show cursor
    print!("{}", concat!("\x1B[?1049l", "\x1B8", "\x1B[?25h"));
}
