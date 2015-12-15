extern crate getopts;

use getopts::Options;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optflag("l", "lines", "print the newline counts");
    opts.optflag("w", "words", "print the word counts");
    opts.optflag("c", "bytes", "print the byte counts");
    opts.optflag("m", "chars", "print the character counts");
    opts.optflag("L",
                 "max-line-length",
                 "print the length of the longest line");
    opts.optflag("h", "help", "display this help and exit");
    opts.optflag("v", "version", "output version information and exit");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => {
            m
        }
        Err(f) => {
            panic!(f.to_string())
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("v") {
        return;
    }

    // just for now.
    let filename = matches.free[0].clone();
    let file = File::open(&filename).ok().expect("I couldn't open that file, sorry :(");
    let reader = BufReader::new(file);

    let mut bytes = 0;
    let mut chars = 0;
    let mut lines = 0;
    let mut words = 0;
    let mut max_line_length = 0;

    for line in reader.lines() {
        let line = line.ok().expect("There was an IO error.");

        lines += 1;
        bytes += line.len();
        bytes += 1; // don't forget the \n!

        words += line.split_whitespace().count();

        let length = line.chars().count();
        chars += length;
        chars += 1; // don't forget the \n!

        if length > max_line_length {
            max_line_length = length;
        }
    }

    if matches.opt_present("l") {
        print!("{}", lines);
    }

    if matches.opt_present("w") {
        print!("{}", words)
    }

    if matches.opt_present("c") {
        print!("{}", bytes)
    }

    if matches.opt_present("m") {
        print!("{}", chars)
    }

    if matches.opt_present("L") {
        print!("{}", max_line_length)
    }

    println!(" {}", filename);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} rwc [options] [<file>]", program);
    print!("{}", opts.usage(&brief));
}
