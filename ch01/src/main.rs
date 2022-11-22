use std::io;

fn main() {
    println!("What is your name?");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Hello, {}, nice to meet you!", &name.trim_end());
}
