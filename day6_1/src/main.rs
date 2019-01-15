use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

struct Coord {
    x: i32,
    y: i32
}

struct Grid {
    matrix: Vec< Vec<Coord> >,
    coords: Vec<Coord>,
    nearest: HashMap<Coord, Coord>
}

impl Grid {
    fn populate_matrix(&mut self, dimension: i32) {
        let bounding_min: i32 = 
        let bounding_max: i32 = 
    }
}

fn get_distance(p: &Coord, q: &Coord) -> i32 {
    (&p.x - &q.x).abs() + (&p.y - &q.y).abs()
}

fn is_infinite(coords: &Vec<Coord>, sample: &Coord) -> bool {

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