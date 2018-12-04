use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut inp = File::open("src/input.txt").expect("File not found!");

    let mut content = String::new();

    inp.read_to_string(&mut content).expect("something went wrong reading the file");

    let lines = content.split("\n");

    let mut total_doubles = 0;
    let mut total_triples = 0;

    for line in lines {
        let mut map: HashMap<char, i32> = HashMap::new();

        for c in line.chars() {
            let mut exists = false;
            match map.get(&c) {
                Some(_val) => {
                    exists = true;
                },
                None => {
                    exists = false;
                }
            }
            if exists == true {
                let prev_val = map[&c];
                map.insert(c, prev_val + 1);
            } else {
                map.insert(c, 1);
            }
        }
        let mut double_found = false;
        let mut triple_found = false;

        for (c, val) in map {
            if val == 2 && double_found == false {
                total_doubles += 1;
                double_found = true;
            }
            if val == 3 && triple_found == false {
                total_triples += 1;
                triple_found = true;
            }
        }
    }

    let total = total_doubles * total_triples;

    println!("doubles: {}", total_doubles);
    println!("triples: {}", total_triples);
    println!("total: {}", total);
}