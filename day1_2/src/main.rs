use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let mut inp = File::open("src/input.txt").expect("File not found!");

    let mut content = String::new();

    inp.read_to_string(&mut content).expect("something went wrong reading the file");

    //let operations = content.split("\n");

    let mut start = 0;

    let mut vals: HashMap<i32, i32> = HashMap::new();

    vals.insert(0, 1);

    let mut duplicate = false;
    let mut duplicate_vals: Vec<i32> = Vec::new();

    while duplicate == false {

        let operations = content.split("\n");

        for i in operations {
            let number: i32 = i.parse().expect("not a number");
            
            start += number;

            match vals.get(&start) {
                Some(_val) => {
                    duplicate = true;
                    duplicate_vals.push(start);
                },
                None => {
                    vals.insert(start, 1);
                }
            }
        }
    }
    println!("first duplicate val: {}", duplicate_vals[0]);
}