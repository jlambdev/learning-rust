// bring additional packages into scope
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // thread_rng -> give random number generator local to current thread of execution
    // gen_range -> pass range expression (start..=end)
    let secret_number = rand::thread_rng().gen_range(1..=100); // rust defaults to i32 return type

    loop {
        println!("Please input your guess.");

        // variables are immutable by default; add `mut` to make it mutable
        // bind empty instance of String to the mutable variable
        let mut guess = String::new();
    
        // creates an instance of std::io::Stdin
        io::stdin()
            .read_line(&mut guess) // reference to mutable variable `guess`
            .expect("Failed to read line"); // `Result` object can be in different states (Ok & Err)
    
        // rust allows shadowing of previous variables (re-use `guess` variable)
        // trim removes \n
        // switch to using `match` to avoid crashing when the user enters a non numeric value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // underscore is a catch-all value, matching all Err values
        };
    
        println!("You guessed: {guess}");
    
        // match expression made up of 'arms'
        // matching pattern to Ordering value returned by `cmp`
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
