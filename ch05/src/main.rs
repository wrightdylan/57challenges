
use std::{io, num::ParseFloatError};

fn test(input: &Result<f64,ParseFloatError>) -> bool {
    match input {
        Ok(ok) => if ok > &0.0 {
                return true
            } else {
                println!("Number is a negative.");
                return false;
            },
        Err(..) => {
            println!("Input is not a number.");
            return false;
        }
    }
}

fn add(f:&f64, s:&f64) -> f64 {
    f + s
}

fn sub(f:&f64, s:&f64) -> f64 {
    f - s
}

fn mult(f:&f64, s:&f64) -> f64 {
    f * s
}

fn div(f:&f64, s:&f64) -> f64 {
    f / s
}

fn main() {
    let first = loop {
        println!("What is the first number?");

        let mut first = String::new();
        io::stdin()
            .read_line(&mut first)
            .expect("Failed to read line.");

        let first_trimmed = first.trim().parse::<f64>();
        if test(&first_trimmed) { break first_trimmed.unwrap(); }
    };

    let second = loop {
        println!("What is the second number?");

        let mut second = String::new();
        io::stdin()
            .read_line(&mut second)
            .expect("Failed to read line.");
        
        let second_trimmed = second.trim().parse::<f64>();
        if test(&second_trimmed) { break second_trimmed.unwrap(); }
    };

    println!("{} + {} = {}", &first, &second, add(&first, &second));
    println!("{} - {} = {}", &first, &second, sub(&first, &second));
    println!("{} * {} = {}", &first, &second, mult(&first, &second));
    println!("{} / {} = {}", &first, &second, div(&first, &second));
}
