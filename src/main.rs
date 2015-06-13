#![feature(libc)]
extern crate libc;

use std::cmp::min;
use std::env::args;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use libc::types::os::arch::c95::c_int;
use libc::consts::os::posix88::SIGINT;
use libc::funcs::posix01::signal::signal;

static mut exit: bool = false;

type Map = Vec<Vec<Cell>>;

struct Cell {
	alive:	bool,
	lives:	bool,
}

fn print_usage() {
	println!("usage: gol genmap SIZE_V SIZE_H  (generates an empty map)");
	println!("       gol play FILE [TIME_MS]   (plays the game)\n");
	println!("       maps: must be rectangular");
	println!("             '·' is a dead cell, anything else is a living cell");
	println!("         ^C: exit");
}

fn print_new_map(size_args: &[String]) {
	let sizev = size_args[0].parse::<usize>().unwrap();
	let sizeh = size_args[1].parse::<usize>().unwrap();
	let map_line = std::iter::repeat('·').take(sizeh).collect::<String>();
	for _ in 0..sizev { println!("{}", map_line); }
}

fn get_map(filename: &String) -> Option<Map> {
	let file = match File::open(filename) {
		Ok(file)	=> file,
		Err(error)	=> { println!("{}: {}", filename, error); return None },
	};
	let reader = BufReader::new(file);
	Some(reader.lines().map(|l| l.unwrap().chars().map(|c| match c { '·' => Cell { alive: false, lives: false }, _ => Cell { alive: true, lives: true } }).collect()).collect())
}

fn iterate(map: &mut Map) {
	let i_max = map.len();
	let j_max = map[0].len();
	let mut live_neighbours: usize;
	for i in 0..i_max { for j in 0..j_max {
		live_neighbours = 0;
		for m in i.saturating_sub(1)..min(i + 2, i_max) {
			for n in j.saturating_sub(1)..min(j + 2, j_max) {
				if map[m][n].alive && (m != i || n !=  j) { live_neighbours += 1; }
			}}
		if !map[i][j].alive { if live_neighbours == 3  { map[i][j].lives = true } }
		else if live_neighbours < 2 || 3 < live_neighbours { map[i][j].lives = false }
	}}
	for i in 0..map.len() { for j in 0..map[0].len() {
		map[i][j].alive = map[i][j].lives;
	}}
}

fn display_map(map: &Map) {
	let mut screen = String::with_capacity(map.len() * (map[0].len() * 3 + 1) + 7);
	screen.push_str("\x1B[2J\x1B[H"); // clears screen and moves cursor to 1;1
	for i in 0..map.len() {
		for j in 0..map[0].len() {
			if map[i][j].alive { screen.push_str(" X"); }
			else { screen.push_str(" ·"); }
		}
		screen.push('\n');
	}
	print!("{}{}x{}, ", screen, map.len(), map[0].len());
}

fn play_map(filename: &String, pause_time_ms: u32) {
	unsafe { let handler = interrupt as *const u64; signal(SIGINT, handler as u64); }
	let mut map = get_map(filename).unwrap();
	let size_row = map[0].len();
	for row in map.iter() { if row.len() != size_row { println!("error: map is not a rectangle"); return } }
	let mut niter = 0usize;
	print!("\x1B[?47h\x1B[?25l"); // saves screen and hides cursor
	loop {
		display_map(&map);
		println!("iteration: {}", niter);
		iterate(&mut map);
		niter += 1;
		std::thread::sleep_ms(pause_time_ms);
		unsafe { if exit { print!("\x1B[?47l\x1B[?25h"); return } } // restores screen and shows cursor
	}
}

fn interrupt(_: c_int) {
	unsafe { exit = true; }
}

fn main() {
	let args: Vec<_> = args().collect();
	if args.len() == 4 && args[1] == "genmap" { print_new_map(&args[2..4]); }
	else if args.len() == 3 && args[1] == "play" { play_map(&args[2], 400); }
	else if args.len() == 4 && args[1] == "play" { play_map(&args[2], args[3].parse::<u32>().unwrap()); }
	else { print_usage(); }
}
