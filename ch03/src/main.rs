use std::io;

fn main() {
    println!("What is the quote?");

    let mut quote = String::new();

    io::stdin()
        .read_line(&mut quote)
        .expect("Failed to read line.");

    println!("Who said it?");

    let mut source = String::new();

    io::stdin()
        .read_line(&mut source)
        .expect("Failed to read line.");

    println!("{} says, \"{}\".", &source.trim(), &quote.trim());
}
