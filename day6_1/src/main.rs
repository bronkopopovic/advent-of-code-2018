use std::fs::File;
use std::io::prelude::*;
//use std::collections::HashMap;

struct Coord {
    x: i32,
    y: i32
}

fn get_distance(p: &Coord, q: &Coord) -> i32 {
    return (&p.x - &q.x).abs() + (&p.y - &q.y).abs();
}

fn main() {
    let mut content = String::new();

    File::open("src/input.txt").expect("File not found!")
        .read_to_string(&mut content).expect("something went wrong reading the file");

    let mut coords: Vec<Coord> = Vec::new();

    for line in content.split("\n") {
        
        let coord = Coord {
            x: line.split(", ")
                .collect::<Vec<_>>()[0]
                .parse::<i32>().expect("not a number"),
            y: line.split(", ")
                .collect::<Vec<_>>()[1]
                .parse::<i32>().expect("not a number"),
        };

        coords.push(coord);
    }

    for c in coords {
        println!("{}, {}", c.x, c.y)
    }
    
}