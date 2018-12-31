use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn adjacent_match(polymer: &Vec<char>, ch: &char, i: usize) -> bool {
    let mut adj_match = false;
    match ch.is_uppercase() {
        true => {
            if !polymer[i+1].is_uppercase() && *ch == polymer[i+1].to_uppercase().collect::<Vec<_>>()[0] {
                adj_match = true;
            }
        },
        false => {
            if !polymer[i+1].is_lowercase() && *ch == polymer[i+1].to_lowercase().collect::<Vec<_>>()[0] {
                adj_match = true;
            }
        }
    }
    adj_match
}

fn react(polymer: &mut Vec<char>) -> usize {
    let mut reaction_finished = false;

    while !reaction_finished {
        
        let mut indices_to_remove_hash: HashMap<usize, bool> = HashMap::new();
        let mut indices_to_remove_vec: Vec<usize> = Vec::new();

        let mut skip_next = false;

        for (i, ch) in polymer.iter().enumerate() {
            if i < polymer.len() - 1 {
                if !skip_next && adjacent_match(&polymer, ch, i) {
                    indices_to_remove_hash.entry(i).or_insert(true);
                    indices_to_remove_hash.entry(i+1).or_insert(true);
                    skip_next = true;
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

    polymer.len()
}

fn improve(polymer: &mut Vec<char>) -> usize {

    let mut test_result: usize;

    let alphabet: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

    let mut shortest_poly: usize = polymer.len();
    
    for ch in alphabet.iter() {

        let mut unit_indices: Vec<usize> = Vec::new();
        
        let mut test_polymer = polymer.clone();

        for (i, n) in test_polymer.iter().enumerate() {
            if ch == n || &ch.to_uppercase().collect::<Vec<_>>()[0] == n {
                unit_indices.push(i.clone());
            }
        }

        unit_indices.sort_by( |a, b| {
            a.cmp(&b)
        });

        let mut iter: usize = 0;
        for i in unit_indices {
            test_polymer.remove(i - iter);
            iter += 1;
        }

        test_result = react(&mut test_polymer);
        
        if test_result < shortest_poly {
            shortest_poly = test_result;
        }
    }
    shortest_poly
}

fn main() {
    let mut content = String::new();

    File::open("src/input.txt").expect("File not found!")
        .read_to_string(&mut content).expect("something went wrong reading the file");

    let mut polymer = content.chars().collect::<Vec<_>>();

    println!("{}", improve(&mut polymer));
    
}