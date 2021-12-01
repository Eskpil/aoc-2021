use std::fs;
use std::path;
use std::env;
use std::io;
use std::io::prelude::*;

#[derive(Debug, Eq, PartialEq)]
enum Measurement {
    NA,
    Increased,
    Decreased
}

pub fn puzzle_one() { 
   let path = env::args().last().unwrap().to_string(); 
   let path = path::Path::new(&path);
   let file = io::BufReader::new(fs::File::open(&path).unwrap());
   let mut report = Vec::<i16>::new();
   let mut measurements = Vec::<Measurement>::new();

   for line in file.lines() {
        let line = line.unwrap();  
        match line.parse::<i16>() {
            Ok(n) => report.push(n),
            Err(err) => eprintln!("error: {:?} when parsing line: {:?}", err, line),
        }
   } 

   for (i, value) in report.iter().enumerate() {
       if i == 0 {
            measurements.push(Measurement::NA);
       } else {
            if report[i - 1] < value.clone() {
                measurements.push(Measurement::Increased);
            } else {
                measurements.push(Measurement::Decreased);
            }
       }
   }

   let mut increased = 0;
   for measurement in measurements.iter() {
        if measurement == &Measurement::Increased {
            increased += 1;
        }
   }
   println!("increased: {:?}", increased);
}

