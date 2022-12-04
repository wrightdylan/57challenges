use std::{io, num::ParseIntError};

fn test(input: &Result<usize,ParseIntError>) -> bool {
    match input {
        Ok(ok) => if ok > &0 {
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

fn cut_pizza(pax: usize, pizzas: usize) -> (usize, usize, usize) {
    let mut slices_per = 4;
    let mut slices = 0;
    let mut leftovers = 0;
    while slices < 1 {
        let total_slices = slices_per * pizzas;
        if total_slices < pax {
            slices_per += 2
        } else {
            slices = total_slices / pax;
            leftovers = total_slices % pax;
        }
    }
    (slices_per, slices, leftovers)
}

// The following functions provide a very basicway of pluralising.
fn pax_noun(num: usize) -> &'static str {
    if num == 1 {
        return "person"
    } else {
        return "people"
    }
}

fn pizza_noun(num: usize) -> &'static str {
    if num == 1 {
        return "pizza"
    } else {
        return "pizzas"
    }
}

fn slice_noun(num: usize) -> &'static str {
    if num == 1 {
        return "slice"
    } else {
        return "slices"
    }
}

fn has_have(num: usize) -> &'static str {
    if num == 1 {
        return "has"
    } else {
        return "have"
    }
}

fn is_are(num: usize) -> &'static str {
    if num == 1 {
        return "is"
    } else {
        return "are"
    }
}

fn each_or(num: usize) -> &'static str {
    if num == 1 {
        return ""
    } else {
        return " each"
    }
}

fn main() {
    let pax = loop {
        println!("How many people?");

        let mut pax = String::new();
        io::stdin()
            .read_line(&mut pax)
            .expect("Failed to read line.");
        
        let pax_trimmed = pax.trim().parse::<usize>();
        if test(&pax_trimmed) { break pax_trimmed.unwrap() };
    };

    let pizzas = loop {
        println!("How many pizzas do you have?");

        let mut pizzas = String::new();
        io::stdin()
            .read_line(&mut pizzas)
            .expect("Failed to read line.");
        
        let pizzas_trimmed = pizzas.trim().parse::<usize>();
        if test(&pizzas_trimmed) { break pizzas_trimmed.unwrap() };
    };

/*
 * The challenge asks to prompt for the number of slices per pizza. This can
 * cause situations where there will be a negative number of slices, which is
 * obviously absurd. Instead I will set a hard limit of a minimum of 4 slices
 * per pizza, and allow slicing each pizza evenly.
 */

    let slices = cut_pizza(pax, pizzas);

    println!("{} {} with {} {}.", pax, pax_noun(pax), pizzas, pizza_noun(pizzas));

    // Pluralisation would be easier with a struct
    println!("Pizzas have a minimum of 4 slices. Your {} {} {} slices{}.", pizza_noun(pizzas), has_have(pizzas), slices.0, each_or(pizzas));
    println!("Each person gets {} {} of pizza.", slices.1, slice_noun(slices.1));
    println!("There {} {} leftover {}.", is_are(slices.2), slices.2, slice_noun(slices.2));
}
