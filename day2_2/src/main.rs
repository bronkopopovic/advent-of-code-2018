use std::fs::File;
use std::io::prelude::*;
//use std::collections::HashMap;

fn main() {
    let mut inp = File::open("src/input.txt").expect("File not found!");

    let mut content = String::new();

    inp.read_to_string(&mut content).expect("something went wrong reading the file");

    let lines1 = content.split("\n").collect::<Vec<&str>>();
    let lines2 = content.split("\n").collect::<Vec<&str>>();

    let mut boxes_found = false;

    for i in 0 .. lines1.len() {
        for j in 0 .. lines2.len() {
            if i != j && !boxes_found {

                let mut misses = 0;
                let mut index = 0;

                let line1: Vec<_> = lines1[i].chars().collect();
                let line2: Vec<_> = lines2[j].chars().collect();
                
                let comparator = line1.iter().zip( line2.iter() );
                
                for (k, chars) in comparator.enumerate() {
                    if chars.0 != chars.1 {
                        misses += 1;
                        index = k;
                    }
                }
                
                let line1copy = line1.clone();
                
                if misses == 1 {

                    boxes_found = true;

                    let mut result = String::new();

                    for l in 0 .. line1copy.len() {
                        if l != index {
                            result.push(line1copy[l]);
                        }
                    }
                    println!("Your result is {}", result);
                }
            }
        }
    }
}
