use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let mut inp = File::open("src/input.txt").expect("File not found!");

    let mut content = String::new();

    inp.read_to_string(&mut content).expect("something went wrong reading the file");

    let operations = content.split("\n");

    let mut start = 0;

    let mut vals: HashMap<i32, i32> = HashMap::new();

    vals.insert(0, 1);

    for i in operations {
        let number: i32 = i.parse().expect("not a number");
        start += number;
        
        let mut occurance = 0;

        match vals.get(&start) {
            Some(_val) => {
                occurance += 1;
                vals.entry(start).or_insert(occurance);
            },
            None => {
                occurance = 1;
                vals.insert(start, occurance);
            }
        }
    }

    for (key, val) in &vals {
        if val >= &2 {
            println!("{} : {}", key, val);
        }
    }
    
}
