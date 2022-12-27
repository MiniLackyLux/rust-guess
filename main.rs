use rand::Rng;
use std::io;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Welcome to the Guessing Game!");
    let mut secret_number: u8 = rand::thread_rng().gen_range(1..101);
    println!("Secret Number = {secret_number}");
    
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error Reading Line");
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Invalid Guess\nGuess is out of range or contains letters");
            continue;},
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "You guessed the correct number!".green());
                println!("Do you want to play again?\nY\nN");
                let mut play_again = String::new();
                io::stdin().read_line(&mut play_again).expect("Error reading input");
                let play_again: String = play_again.trim().to_lowercase();
                if play_again != "y" {
                    println!("Bye!");
                    break;
                }
                else {
                    secret_number = rand::thread_rng().gen_range(1..101);
                }
            },
            Ordering::Less => println!("{}", "You guesses less than the correct number!".red()),
            Ordering::Greater => println!("{}", "You guessed more than the correct number!".red()),
            
        };

    }
}
