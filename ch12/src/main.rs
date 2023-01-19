use std::{io, num::{ParseFloatError, ParseIntError}};

fn get_principal() -> f32 {
    loop {
        println!("Enter the principal:");

        let mut principal = String::new();
        io::stdin()
            .read_line(&mut principal)
            .expect("Failed to read line.");
        
        let principal_trim = principal.trim().parse::<f32>();
        if test_fin_float(&principal_trim) { break principal_trim.unwrap() };
    }
}

fn get_rate() -> f32 {
    loop {
        println!("Enter interest rate as %:");

        let mut rate = String::new();
        io::stdin()
            .read_line(&mut rate)
            .expect("Failed to read line.");

        let rate_trim = rate.trim().parse::<f32>();
        if test_float(&rate_trim) { break rate_trim.unwrap() };
    }
}

fn get_years() -> usize {
    loop {
        println!("Enter whole number of years:");

        let mut years = String::new();
        io::stdin()
            .read_line(&mut years)
            .expect("Failed to read line.");
        
        let years_trim = years.trim().parse::<usize>();
        if test_int(&years_trim) { break years_trim.unwrap() };
    }
}

// Because this is 'simple interest' it does not take into account compounding like in real life
fn calculate_simple_interest(principal: f32, rate: f32, years: usize) -> f32 {
    let interest = principal + (principal * (rate / 100.0) * years as f32);
    interest
}

fn test_fin_float(input: &Result<f32, ParseFloatError>) -> bool {
    match input {
        Ok(ok) => if ok > &0.0 {
                if ok.to_string().contains('.') && ok.to_string().split('.').last().unwrap().len() > 2 {
                    println!("Number must not have more than 2 decimal places.");
                    return false;
                } else {
                    return true;
                }
            } else {
                println!("Number must positive.");
                return false;
            },
        Err(..) => {
            println!("Input is not a number.");
            return false;
        }
    }
}

fn test_float(input: &Result<f32, ParseFloatError>) -> bool {
    match input {
        Ok(ok) => if ok > &0.0 {
                return true;
            } else {
                println!("Number must positive.");
                return false;
            },
        Err(..) => {
            println!("Input is not a number.");
            return false;
        }
    }
}

fn test_int(input: &Result<usize, ParseIntError>) -> bool {
    match input {
        Ok(ok) => if ok > &0 {
                return true
            } else {
                println!("Number must be a positive whole number.");
                return false;
            },
        Err(..) => {
            println!("Input is not a number.");
            return false;
        }
    }
}

fn main() {
    let principal = get_principal();
    let rate = get_rate();
    let years = get_years();

    for i in 1..=years {
        let interest = calculate_simple_interest(principal, rate, i);
        if i == 1 {
            println!("After {} year at {}%, the investment will be worth ${}.", i, rate, interest);
        } else {
            println!("After {} years at {}%, the investment will be worth ${}.", i, rate, interest);
        }
    }
}
