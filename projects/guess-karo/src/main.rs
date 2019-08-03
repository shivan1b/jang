extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("!!!! GUESS KARO GAME !!!!");

    // "thread_rng" gives the rand num generator
    // "gen_range" is called on the rand num gen
    // this method is defined by the "Rng" trait
    let hidden_num = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("\nWhich number do you think I have hidden?");

        // Create an empty string and bind it to "guess" variable.
        // "new" is an associated method of "String" type.
        // Associated methods can only be used on types and not objects.
        // These are also called "static methods" in other languages.
        let mut guess = String::new();

        // read from standard input into the reference of "guess".
        // read_line returns "Result" types which happen to be enums.
        // Enums can have fixed number of values called "variants".
        // The variants available here are Ok and Err.
        // In case of Ok, the value is returned as-is.
        // In case of Err, the "expect" method shall be called.
        io::stdin().read_line(&mut guess)
            .expect("Uh-oh! Are you sure you entered the right kind of value?");

        // Shadow the guess variable to change its type to u32.
        // Annonate the exact type expected.
        // "trim" removes any extra whitespace in the beginning or the end.
        // "parse" parses string into some kind of number, here u32.
        //
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // "cmp" as the name suggests compares two values
        // "Ordering" just like "Result" is an enum with variants available
        // being "Less", "Equal" and "Greater".
        // "match" decides what to do based on a variant.
        match guess.cmp(&hidden_num) {
            Ordering::Less => println!("Move a bit higher?"),
            Ordering::Equal => {
                println!("Bravo! You guessed it!");
                break;
            }
            Ordering::Greater => println!("Move a little lower?"),
        }
    }
}
