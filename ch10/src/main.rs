use std::{io, num::{ParseFloatError, ParseIntError}};

fn num_item_type() -> usize {
    loop {
        println!("Enter the number of item types:");

        let mut num_item_type = String::new();
        io::stdin()
            .read_line(&mut num_item_type)
            .expect("Failed to read line.");

        let num_item_type_trimmed = num_item_type.trim().parse::<usize>();
        if test_int(&num_item_type_trimmed) { break num_item_type_trimmed.unwrap(); }
    }
}

fn item_price(index: usize) -> f64 {
    loop {
        println!("Enter the price for item {}:", index + 1);

        let mut item_price = String::new();
        io::stdin()
            .read_line(&mut item_price)
            .expect("Failed to read line.");

        let item_price_trimmed = item_price.trim().parse::<f64>();
        if test_price(&item_price_trimmed) { break item_price_trimmed.unwrap(); }
    }
}

fn item_quantity(index: usize) -> usize {
    loop {
        println!("Enter the quantity for item {}:", index + 1);

        let mut item_quantity = String::new();
        io::stdin()
            .read_line(&mut item_quantity)
            .expect("Failed to read line.");

        let item_quantity_trimmed = item_quantity.trim().parse::<usize>();
        if test_int(&item_quantity_trimmed) { break item_quantity_trimmed.unwrap(); }
    }
}

fn test_int(input: &Result<usize,ParseIntError>) -> bool {
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

// TODO: Test to ensure price does not have more than 2dp
fn test_price(input: &Result<f64,ParseFloatError>) -> bool {
    match input {
        Ok(ok) => if ok > &0.0 {
                return true
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

fn main() {
    let mut item_totals = Vec::new();
    let num_item_type = num_item_type() as usize;
    for n in 0..num_item_type {
        let item_quantity = item_quantity(n) as f64;
        let item_price = item_price(n);
        item_totals.push(item_quantity * item_price);
    }
    
    let subtotal: f64 = item_totals.iter().sum();
    println!("Subtotal: ${:.2}", subtotal);
    let tax = subtotal * 0.055;
    println!("Tax: ${:.2}", tax);
    let total = subtotal + tax;
    println!("Total: ${:.2}", total);
}
