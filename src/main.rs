#![feature(io)]

use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;

use docopt::Docopt;

static USAGE: &'static str = "
Usage: rwc [options] [<file>]

Options:
    -c, --bytes            print the byte counts
    -m, --chars            print the character counts
    -l, --lines            print the newline counts
    -w, --words            print the word counts
    -L, --max-line-length  print the length of the longest line
    -h, --help             display this help and exit
    -v, --version          output version information and exit
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_file: Option<String>,
    flag_bytes: bool,
    flag_chars: bool,
    flag_lines: bool,
    flag_words: bool,
    flag_max_line_length: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    // just for now.
    let filename = args.arg_file.unwrap();

    if args.flag_bytes {
        let file = File::open(&filename).ok().expect("I couldn't open that file, sorry :(");

        let bytes = file.bytes()
            .map(|x| x.ok().expect("There was an IO error."))
            .count();

        println!("bytes: {}", bytes);
    }
    if args.flag_chars {
        let file = File::open(&filename).ok().expect("I couldn't open that file, sorry :(");

        let chars = file.chars()
            .map(|x| x.ok().expect("There was an IO error."))
            .count();

        println!("chars: {}", chars);

    }
    if args.flag_lines {
        let file = File::open(&filename).ok().expect("I couldn't open that file, sorry :(");

        let newlines = file.bytes()
            .map(|x| x.ok().expect("There was an IO error."))
            .filter(|x| *x == b'\n')
            .count();

        println!("newlines: {}", newlines);
    }
    if args.flag_words {
        let file = File::open(&filename).ok().expect("I couldn't open that file, sorry :(");

        let reader = BufReader::new(file);

        let words = reader.split(b' ')
            .map(|x| x.ok().expect("There was an IO error."))
            .count();

        println!("words: {}", words);
    }
    if args.flag_max_line_length {
        let file = File::open(&filename).ok().expect("I couldn't open that file, sorry :(");

        let reader = BufReader::new(file);

        let longest_line = reader.lines()
            .map(|x| x.ok().expect("There was an IO error."))
            .map(|x| x.len())
            .max().unwrap();

        println!("longest_line: {}", longest_line);
    }
}

