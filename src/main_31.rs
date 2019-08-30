// task08
// Number Guessing Game
// Build a number-guessing game to generate a random integer number from 1 to 10 and have the user try to guess
// that. Tell them if they get it right or wrong, and if they get it wrong, show them what the random number was.
// Program Console Sample 1:
// I'm thinking of a number from 1 to 10.
// Your guess: 3
// Sorry, but I was really thinking of 4.
// Program Console Sample 2:
// I'm thinking of a number from 1 to 10.
// Your guess: 4
// Sorry, but I was really thinking of 7.
// Program Console Sample 3:
// I'm thinking of a number from 1 to 10.
// Your guess: 2
// That's right! My secret number was 2!

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("I'm thinking of a number from 1 to 10");


    loop {
        let secret_number = rand::thread_rng().gen_range(1, 11);
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("That's right! My secret number was {}!",&secret_number);
                break;
            }
        }
    }
}

