use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut inp = File::open("src/input.txt").expect("File not found!");

    let mut content = String::new();

    inp.read_to_string(&mut content).expect("something went wrong reading the file");

    let operations = content.split("\n");

    let mut start = 0;

    for i in operations {
        let number: i32 = i.parse().expect("not a number");
        start += number;
    }

    println!("{}", start);
}