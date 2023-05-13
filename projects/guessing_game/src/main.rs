// bring additional packages into scope
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // thread_rng -> give random number generator local to current thread of execution
    // gen_range -> pass range expression (start..=end)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    // variables are immutable by default; add `mut` to make it mutable
    // bind empty instance of String to the mutable variable
    let mut guess = String::new();

    // creates an instance of std::io::Stdin
    io::stdin()
        .read_line(&mut guess) // reference to mutable variable `guess`
        .expect("Failed to read line"); // `Result` object can be in different states (Ok & Err)

    println!("You guessed: {guess}");
}
