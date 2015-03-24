#![feature(str_words)]

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
        words += line.words().count();

        let length = line.chars().count();
        chars += length;
        chars += 1; // don't forget the \n!

        if length > max_line_length {
            max_line_length = length;
        }
    }

    println!("{} {} {} {}\n", lines, words, bytes, filename);
}

