use std::collections::HashMap;
use std::{io,fs};
use std::io::{BufRead, Write};
use io::BufReader;

fn main() {
    let mut input = fs::File::open("energie.in").unwrap();
    let mut output = fs::OpenOptions::new().write(true).open("energie.out").unwrap();
    let mut out:f32 = 0.0;
    let mut n:f32 = 0.0;
    for i in BufReader::new(input).lines().skip(1) {
        let i = match i {
            Err(_) => break,
            Ok(i) => i
        };
        n += 1.0;
        out += i.split(' ').last().unwrap().parse::<f32>().unwrap();
    };
    output.write_all( (out/n).to_string().as_bytes());
}
