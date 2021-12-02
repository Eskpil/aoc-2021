use std::fs;
use std::path;
use std::env;
use std::io;
use std::io::prelude::*;

pub fn puzzle() {
    let path = env::args().nth(2).unwrap().to_string();
    let path = path::Path::new(&path);
    let file = io::BufReader::new(fs::File::open(&path).unwrap());
    
    let mut forward = 0;
    let mut depth = 0;

    for line in file.lines() {
        let line = line.unwrap();
        let mut values = line.split(" ");
        let direction = values.next().unwrap();
        let value = values.next().unwrap().parse::<usize>().unwrap();

        if direction == "forward" {
            forward += value;
       } else if direction == "down" {
            depth += value; 
        } else {
            depth -= value; 
        }
    }

    println!("result: {}", depth * forward);
}
