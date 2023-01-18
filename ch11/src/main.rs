use std::{io, num::ParseFloatError};
use tokio::time::error::Error;
use reqwest::header::*;

// api-key file should not have a new line
const API_KEY: &'static str = include_str!("../api-key");

fn amount_from() -> f64 {
    loop {
        println!("How many Euros are you exchanging?");

        let mut amount_from = String::new();
        io::stdin()
            .read_line(&mut amount_from)
            .expect("Failed to read line.");

        let amount_from_trimmed = amount_from.trim().parse::<f64>();
        if test_input(&amount_from_trimmed) { break amount_from_trimmed.unwrap(); }
    }
}

// Deprecated function since the exchange rate is obtained via API
/*
fn exchange_rate() -> f64 {
    loop {
        println!("What is the exchange rate?");

        let mut exchange_rate = String::new();
        io::stdin()
            .read_line(&mut exchange_rate)
            .expect("Failed to read line.");

        let exchange_rate_trimmed = exchange_rate.trim().parse::<f64>();
        if test_input(&exchange_rate_trimmed) { break exchange_rate_trimmed.unwrap(); }
    }
}
*/

fn test_input(input: &Result<f64,ParseFloatError>) -> bool {
    match input {
        Ok(ok) => if ok > &0.00 {
                return true
            } else {
                println!("Number must be positive.");
                return false;
            },
        Err(..) => {
            println!("Input is not a number.");
            return false;
        }
    }
}

// Not that robust, but it works
fn round_up(input: f64) -> f64 {
    if input % 0.01 > 0.000 {
        input - (input % 0.01) + 0.01
    } else {
        input
    }
}

// A basic program like this isn't a challenge. Let's do the real challenge using a live API
/*
fn main() {
    let amount_from = amount_from();
    let rate = exchange_rate();

    let amount_to = round_up(amount_from / rate);

    println!("{} Euros at an exchange rate of {} is {:.2} US Dollars.", amount_from, rate, amount_to);
}
*/

// Async isn't appropriate for single requests like this, but this is just for the challenge.
#[tokio::main]
async fn main() -> Result<(), Error> {
    let amount_from = amount_from();
    
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.apilayer.com/exchangerates_data/latest?base=EUR&symbols=USD");
    let response = client
        .get(url)
        .header("apikey", API_KEY)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    let symbol = "USD";
    let rate = response.pointer("/rates")
        .unwrap()
        .get(symbol)
        .unwrap()
        .as_f64()
        .unwrap();

    let amount_to = round_up(amount_from / rate);
    println!("{} Euros at an exchange rate of {} is {:.2} US Dollars.", amount_from, rate, amount_to);

    Ok(())
}