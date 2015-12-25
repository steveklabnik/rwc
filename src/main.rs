extern crate getopts;

use getopts::Options;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader, stdin};

struct CountResults {
    bytes: u64,
    chars: u64,
    lines: u64,
    words: u64,
    max_line_length: u64,
}

fn do_count<R: Read>(reader: BufReader<R>) -> CountResults {
    let mut res = CountResults { bytes: 0, chars: 0, lines: 0,
                                 words: 0, max_line_length: 0 };

    for line in reader.lines() {
        let line = line.ok().expect("There was an IO error.");

        res.lines += 1;
        res.bytes += line.len() as u64;
        res.bytes += 1; // don't forget the \n!

        res.words += line.split_whitespace().count() as u64;

        let length = line.chars().count() as u64;
        res.chars += length;
        res.chars += 1; // don't forget the \n!

        if length > res.max_line_length {
            res.max_line_length = length;
        }
    }

    res
}

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

    let (counts, filename) = match matches.free.len() {
        0 => (do_count(BufReader::new(stdin())), "<stdin>".to_string()),
        1 => {
            // just for now.
            let filename = matches.free[0].clone();
            let file = File::open(&filename).ok().expect("I couldn't open that file, sorry :(");
            (do_count(BufReader::new(file)), filename)
        },
        _ => panic!("A single filename was expected"),
    };

    if matches.opt_present("lines") {
        print!("{} ", counts.lines);
    }

    if matches.opt_present("words") {
        print!("{} ", counts.words)
    }

    if matches.opt_present("bytes") {
        print!("{} ", counts.bytes)
    }

    if matches.opt_present("chars") {
        print!("{} ", counts.chars)
    }

    if matches.opt_present("max-line-length") {
        print!("{} ", counts.max_line_length)
    }

    println!(" {}", filename);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} rwc [options] [<file>]", program);
    print!("{}", opts.usage(&brief));
}
