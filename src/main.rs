#![feature(core)]

extern crate core;

use std::env;

mod emu;

fn main() {
    match parse_args(env::args()) {
	Ok(arg) => emu::run(&arg),
        Err(err) => println!("{}", err),
    }
}

fn parse_args(argv: env::Args) -> Result<String, &'static str> {
	let result: Result<String, &'static str>;
	
	let mut name_skipped_argv = argv.skip(1);
	
	if let Some(arg) = name_skipped_argv.next() {
		if let Some(_) = name_skipped_argv.next() {
			result = Err("Only need one argument");
		} else {
			result = Ok(arg);
		}
	} else {
		result = Err("Need at least one argument.");
	}
		
	result
}

