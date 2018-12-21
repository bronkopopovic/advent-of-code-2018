use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut content = String::new();

    File::open("src/input.txt").expect("File not found!")
        .read_to_string(&mut content).expect("something went wrong reading the file");

    let mut start = 0;

    for i in content.split("\n") {
        let number: i32 = i.parse().expect("not a number");
        start += number;
    }

    println!("{}", start);
}