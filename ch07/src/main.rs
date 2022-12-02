use std::{io, num::ParseFloatError};

fn test(input: &Result<f64,ParseFloatError>) -> bool {
    match input {
        Ok(ok) => if ok > &0.0 {
                return true
            } else {
                println!("Number must be non-zero and positive.");
                return false;
            },
        Err(..) => {
            println!("Input is not a number.");
            return false;
        }
    }
}

fn unit_name(is_metric: &bool) -> &str {
    if *is_metric {
        return "metres";
    } else {
        return "feet";
    }
}

fn calc_area(length: f64, width: f64) -> f64 {
    length * width
}

fn convert(area: f64, is_metric: bool, metric: bool) -> f64 {
    const SQFT_TO_SQM: f64 = 0.09290304;

    if !is_metric && metric {
        area * SQFT_TO_SQM
    } else if is_metric && !metric {
        area * (1.0/SQFT_TO_SQM)
    } else {
        // No conversion necessary
        area
    }
}

fn main() {
    let is_metric: bool = loop {
        println!("Choose input units: (f)eet or (m)etres:");

        let mut unit = String::new();
        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line.");
        
        let unit_trimmed = unit.trim()
                                    .to_lowercase()
                                    .chars()
                                    .nth(0)
                                    .unwrap();
        match unit_trimmed {
            'f' => break false,
            'm' => break true,
            _ => println!("Option must be either \'f\' or \'m\'.")
        }
    };

    let unit_name = unit_name(&is_metric);

    let length = loop {
        println!("What is the length of the room in {}?", &unit_name);

        let mut length = String::new();
        io::stdin()
            .read_line(&mut length)
            .expect("Failed to read line.");

        let length_trimmed = length.trim().parse::<f64>();
        if test(&length_trimmed) { break length_trimmed.unwrap(); }
    };

    let width = loop {
        println!("What is the width of the room in {}?", &unit_name);

        let mut width = String::new();
        io::stdin()
            .read_line(&mut width)
            .expect("Failed to read line.");
        
        let width_trimmed = width.trim().parse::<f64>();
        if test(&width_trimmed) { break width_trimmed.unwrap(); }
    };

    println!("You entered dimensions of {} {} by {} {}.", length, unit_name, width, unit_name);

    let area = calc_area(length, width);

    println!("The area is:");
    println!("{} square feet", convert(area, is_metric, false));
    println!("{} square metres", convert(area, is_metric, true));
    // I'm 100% sure there's a far better way of doing this. I'll have to refactor when I gain more experience.
}
