use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The Secret number is: {secret_number}"); // here only for debugging
                                                       // will be removed in real game
    println!("Please input your Guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please enter a number!");
    // u32 is the default
    // trim method will eliminate any whitespaces at the beginning and the end
    // parse method converts string into another type (u32 integer sepcified here!)
    // expect method is used again in case the input wasn't a number (to prevent "result" failure)

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("You win!"),
    }

    //TODO:looping to allow multiple guesses
}
