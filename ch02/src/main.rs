use std::io;

fn main() {
    println!("What is the input string?");

    let mut your_string = String::new();

    io::stdin()
        .read_line(&mut your_string)
        .expect("Failed to read line.");
    
    let your_string = your_string.trim();

    if your_string.is_empty() {
        println!("User did not enter anything.");
    } else {
        println!("{} has {} characters.", &your_string, &your_string.len());
    }
}
