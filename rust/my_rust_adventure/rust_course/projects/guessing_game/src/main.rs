use std::cmp::Ordering;
use std::io; //use standard library, input and output library

use rand::Rng; //use rand, random number generator trait

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101); /*
                                                              generate a random number using the thread_rng function,
                                                              between 1 and 100
                                                              */
    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); /*
                                       create mutable empty string
                                       variable to allow input from user
                                       */
        io::stdin() /*
            reads the guess input from user and returns
            error if something went wrong
            */
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; /* taking the previous guess input and using
           the trim method to remove any whitespace. the parse method is then
           used to transform the guess string into the desired type,
           in this case u32. match is used for error handling
           */
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            /*
            using the compare method on user's guess input
            to compare with secret number and match on how close
            the guess is to the secret number
            */
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}