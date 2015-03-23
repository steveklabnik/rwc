#![feature(io)]

use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args[1].clone();

    let file = File::open(&filename).ok().expect("I couldn't open that file, sorry :(");

    let reader = BufReader::new(file);

    let words = reader.split(b' ')
        .map(|x| x.ok().expect("There was an IO error."))
        .count();

    println!("words: {}", words);

    let file = File::open(&filename).ok().expect("I couldn't open that file, sorry :(");

    let newlines = file.bytes()
        .map(|x| x.ok().expect("There was an IO error."))
        .filter(|x| *x == b'\n')
        .count();

    println!("newlines: {}", newlines);

    let file = File::open(&filename).ok().expect("I couldn't open that file, sorry :(");

    let bytes = file.bytes()
        .map(|x| x.ok().expect("There was an IO error."))
        .count();

    println!("bytes: {}", bytes);

    let file = File::open(&filename).ok().expect("I couldn't open that file, sorry :(");

    let chars = file.chars()
        .map(|x| x.ok().expect("There was an IO error."))
        .count();

    println!("chars: {}", chars);

    let file = File::open(&filename).ok().expect("I couldn't open that file, sorry :(");

    let reader = BufReader::new(file);

    let longest_line = reader.lines()
        .map(|x| x.ok().expect("There was an IO error."))
        .map(|x| x.len())
        .max().unwrap();

    println!("longest_line: {}", longest_line);
}

