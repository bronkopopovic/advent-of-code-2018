extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut content = String::new();

    File::open("src/input.txt").expect("File not found!")
        .read_to_string(&mut content).expect("something went wrong reading the file");

    let mut claims: Vec<( i32, (i32, i32), (i32, i32) )> = Vec::new();

    let re = Regex::new(r"(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();
    
    for line in content.split("\n") {        
        let caps = re.captures(&line).unwrap();

        claims.push( (caps[1].parse::<i32>().expect("not a number"), 
            (caps[2].parse::<i32>().expect("not a number"), caps[3].parse::<i32>().expect("not a number")), 
            (caps[4].parse::<i32>().expect("not a number"), caps[5].parse::<i32>().expect("not a number"))
        ) );
    }

    let mut fabric: HashMap<(i32, i32), bool> = HashMap::new();

    let mut overlaps = 0;

    for claim in &claims {
        for x in (claim.1).0 .. (claim.1).0 + (claim.2).0 {
            for y in (claim.1).1 .. (claim.1).1 + (claim.2).1 {

                let overlaps_on_coord: bool;
                let coord_exists: bool;
                
                match fabric.get( &(x, y) ) {
                    Some(_val) => {
                        coord_exists = true;
                        overlaps_on_coord = *_val;
                    },
                    None => {
                        coord_exists = false;
                        overlaps_on_coord = false;
                    }
                }
                if !coord_exists {
                    fabric.insert( (x, y), false );
                } else {
                    if !overlaps_on_coord {
                        overlaps += 1;
                        fabric.insert( (x, y), true );
                    }
                }
            }
        }
    }
    for claim in &claims {
        
        let mut claim_overlaps = 0;
        
        for x in (claim.1).0 .. (claim.1).0 + (claim.2).0 {
            for y in (claim.1).1 .. (claim.1).1 + (claim.2).1 {

                let coord_overlaps: bool;

                match fabric.get( &(x, y) ) {
                    Some(_val) => {
                        coord_overlaps = *_val;
                    }, 
                    None => {
                        coord_overlaps = false;
                    }
                }
                if coord_overlaps {
                    claim_overlaps += 1;
                }
            }
        }
        if claim_overlaps == 0 {
            println!("ID without overlaps: {}", claim.0);
        }
    }
}
