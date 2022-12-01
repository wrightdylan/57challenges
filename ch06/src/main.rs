use std::{io, num::ParseIntError};
use chrono::prelude::*;

fn test(input: &Result<i32,ParseIntError>) -> bool {
    match input {
        Ok(ok) => if ok > &0 {
                return true
            } else {
                println!("Age must be non-zero and positive.");
                return false;
            },
        Err(..) => {
            println!("Input is not a number.");
            return false;
        }
    }
}

fn calc_retire(age: i32, retire: i32) {
    let years_left = retire - age;
    if &years_left > &0 {
        println!("You have {} years left until you can retire.", &years_left);
        let dt = Utc::now();
        println!("It's {}, so you can retire in {}.", dt.year(), dt.year() + &years_left);
    } else {
        println!("You can already retire!");
    }
}

fn main() {
    let age = loop {
        println!("What is your current age?");

        let mut age = String::new();
        io::stdin()
            .read_line(&mut age)
            .expect("Failed to read line.");
        
        let age_trimmed = age.trim().parse::<i32>();
        if test(&age_trimmed) { break age_trimmed.unwrap() };
    };

    let retire = loop {
        println!("At what age would you like to retire?");

        let mut retire = String::new();
        io::stdin()
            .read_line(&mut retire)
            .expect("Failed to read line.");
        
        let retire_trimmed = retire.trim().parse::<i32>();
        if test(&retire_trimmed) { break retire_trimmed.unwrap() };
    };

    calc_retire(age, retire);
}
