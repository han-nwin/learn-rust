use std::cmp::Ordering;
use std::io;

use rand::RngExt;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mutable var

        // Read user input
        // Result's variants are Ok and Err. If Ok -> take the output of Ok
        // if Err -> need to handle error
        // The right way to suppress the warning
        // is to actually write error-handling code,
        // but in our case we just want to crash this program when a problem occurs,
        // so we can use expect
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // type conversion to unsigned 32-bit number
        // Rust allows us to shadow the previous value of
        // guess with a new one. Shadowing lets us reuse the guess variable
        // name rather than forcing us to create two unique variables
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => println!("You win!"),
            Ordering::Greater => println!("Too big"),
        }
    }
}
