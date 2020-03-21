use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::prelude::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Guess a number between 1 and 100!");

    loop {
        print!("Please enter your guess: ");
        io::stdout().flush().expect("Failed to flush buffer");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small, try again!"),
            Ordering::Greater => println!("Too big, try again!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
