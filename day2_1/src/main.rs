use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut content = String::new();

    File::open("src/input.txt").expect("File not found!")
        .read_to_string(&mut content).expect("something went wrong reading the file");

    let mut total_doubles = 0;
    let mut total_triples = 0;

    for line in content.split("\n") {
        let mut map: HashMap<char, i32> = HashMap::new();

        for c in line.chars() {
            let mut c_exists;
            match map.get(&c) {
                Some(_val) => {
                    c_exists = true;
                },
                None => {
                    c_exists = false;
                }
            }
            if c_exists == true {
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