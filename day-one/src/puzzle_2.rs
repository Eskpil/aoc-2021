use std::fs;
use std::path;
use std::env;
use std::io;
use std::io::prelude::*;

#[derive(Debug, Eq, PartialEq)]
enum Measurement {
    NA,
    Increased,
    NoChange,
    Decreased
}

pub fn puzzle_two() {
   let path = env::args().last().unwrap().to_string(); 
   let path = path::Path::new(&path);
   let file = io::BufReader::new(fs::File::open(&path).unwrap());
   let mut report = Vec::<i16>::new();
   let mut measurements = Vec::<Measurement>::new();
   let mut groups = Vec::<i16>::new();

   for line in file.lines() {
        let line = line.unwrap();  
        match line.parse::<i16>() {
            Ok(n) => report.push(n),
            Err(err) => eprintln!("error: {:?} when parsing line: {:?}", err, line),
        }
   }  

   let windows = report.windows(3);
   
   for window in windows {
        let mut group = 0;
        for value in window.iter() {
           group += value.clone(); 
        }
        groups.push(group);
   }

   for (i, value) in groups.iter().enumerate() {
       if i == 0 {
            measurements.push(Measurement::NA);
       } else {
            if groups[i - 1] == value.clone() {
                measurements.push(Measurement::NoChange);
            } else if groups[i - 1] < value.clone() {
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

/* *
 * 199 A
 * 200 A B
 * 208 A B C
 * 210   B C D
 * 200     C D E
 * 207       D E F
 * 240         E F G
 * 269           F G H
 * 260             G H
 * 263               H
 */
