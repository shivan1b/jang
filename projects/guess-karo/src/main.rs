use std::io;

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

    println!("You guessed: {}", guess);
}
