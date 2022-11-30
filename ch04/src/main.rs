use std::io;

fn main() {
    println!("Enter a noun: ");

    let mut noun = String::new();

    io::stdin()
        .read_line(&mut noun)
        .expect("Failed to read line.");

    println!("Enter a verb: ");

    let mut verb = String::new();

    io::stdin()
        .read_line(&mut verb)
        .expect("Failed to read line.");
    
    println!("Enter a adjective: ");

    let mut adjective = String::new();

    io::stdin()
        .read_line(&mut adjective)
        .expect("Failed to read line.");

    println!("Enter a adverb: ");

    let mut adverb = String::new();

    io::stdin()
        .read_line(&mut adverb)
        .expect("Failed to read line.");

    println!("Do you {} your {} {} {}? That's hilarious!", &verb.trim(), &adjective.trim(), &noun.trim(), &adverb.trim());

    // This is an awful solution; too much repetition.
    // Maybe use an enum and vector for a DRY approach
}
