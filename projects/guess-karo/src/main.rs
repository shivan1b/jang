extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("!!!! GUESS KARO GAME !!!!");
    println!("Which number do you think I have hidden?");

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

    // "thread_rng" gives the rand num generator
    // "gen_range" is called on the rand num gen
    // this method is defined by the "Rng" trait
    let hidden_num = rand::thread_rng().gen_range(1, 101);

    println!("You guessed: {}", guess);
    println!("Aaaaaand the hidden number was: {}", hidden_num);
}
