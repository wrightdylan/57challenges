use std::{io, num::{ParseFloatError, ParseIntError}};

fn get_principal() -> f64 {
    loop {
        println!("Enter the principal:");

        let mut principal = String::new();
        io::stdin()
            .read_line(&mut principal)
            .expect("Failed to read line.");
        
        let principal_trim = principal.trim().parse::<f64>();
        if test_fin_float(&principal_trim) { break principal_trim.unwrap() };
    }
}

fn get_rate() -> f64 {
    loop {
        println!("Enter interest rate as %:");

        let mut rate = String::new();
        io::stdin()
            .read_line(&mut rate)
            .expect("Failed to read line.");

        let rate_trim = rate.trim().parse::<f64>();
        if test_float(&rate_trim) { break rate_trim.unwrap() };
    }
}

fn get_freq() -> usize {
    loop {
        println!("Enter the number of times the interest is compounded per year:");

        let mut freq = String::new();
        io::stdin()
            .read_line(&mut freq)
            .expect("Failed to read line.");
        
        let freq_trim = freq.trim().parse::<usize>();
        if test_int(&freq_trim) { break freq_trim.unwrap() };
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

fn calculate_compound_interest(principal: f64, rate: f64, freq: usize, years: usize) -> f64 {
    let exp = (freq * years) as f64;
    let interest = principal * (1.0 + ((rate / 100.0) / freq as f64)).powf(exp);
    interest
}

fn test_fin_float(input: &Result<f64, ParseFloatError>) -> bool {
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

fn test_float(input: &Result<f64, ParseFloatError>) -> bool {
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
    let freq = get_freq();
    let years = get_years();

    for i in 1..=years {
        let interest = calculate_compound_interest(principal, rate, freq, i);
        if i == 1 {
            println!("After {} year at {}%, the investment will be worth ${:.2}.", i, rate, interest);
        } else {
            println!("After {} years at {}%, the investment will be worth ${:.2}.", i, rate, interest);
        }
    }
}
