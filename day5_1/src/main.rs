use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn adjacent_match(polymer: &Vec<char>, ch: &char, i: usize) -> bool {
    match ch.is_uppercase() {
        true => {
            if !polymer[i+1].is_uppercase() {
                if *ch == polymer[i+1].to_uppercase().collect::<Vec<_>>()[0] {
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        },
        false => {
            if !polymer[i+1].is_lowercase() {
                if *ch == polymer[i+1].to_lowercase().collect::<Vec<_>>()[0] {
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
}

fn main() {
    let mut content = String::new();

    File::open("src/input.txt").expect("File not found!")
        .read_to_string(&mut content).expect("something went wrong reading the file");

    let mut polymer = content.chars().collect::<Vec<_>>();

    let mut reaction_finished = false;

    while !reaction_finished {
        
        let mut indices_to_remove_hash: HashMap<usize, bool> = HashMap::new();
        let mut indices_to_remove_vec: Vec<usize> = Vec::new();

        let mut skip_next = false;

        for (i, ch) in polymer.iter().enumerate() {
            if i < polymer.len() - 1 {
                if !skip_next {
                    if adjacent_match(&polymer, ch, i) {
                        indices_to_remove_hash.entry(i).or_insert(true);
                        indices_to_remove_hash.entry(i+1).or_insert(true);
                        skip_next = true;
                    } else {
                        skip_next = false;
                    }
                } else {
                    skip_next = false;
                }
            }
        }

        if indices_to_remove_hash.len() > 0 {
            for ch in indices_to_remove_hash.keys() {
                indices_to_remove_vec.push(*ch);
            }

            indices_to_remove_vec.sort_by( |a, b| {
                a.cmp(&b)
            });

            let mut delete_iterations: usize = 0;
            for ch in indices_to_remove_vec {
                polymer.remove(ch - delete_iterations);
                delete_iterations += 1;
            }
        } else {
            reaction_finished = true;
        }
    }

    println!("\n\nfinal length: {}", polymer.len());
}