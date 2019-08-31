// task10
// Hi-Lo with Limited Tries
// Write a program that picks a random number from 1-100. The user keeps guessing as long as their guess is wrong,
// and they've guessed less than 7 times. If their guess is higher than the number, say "Too high." If their guess is
// lower than the number, say "Too low." When they get it right, the game stops. Or, if they hit seven guesses, the
// game stops even if they never got it right.
// Program Console Sample 1:
// I'm thinking of a number between 1-100. You have 7 guesses.
// First guess: 50
// Sorry, you are too low.
// Guess # 2: 75
// Sorry, you are too low.
// Guess # 3: 87
// Sorry, that guess is too high.
// Guess # 4: 82
// Sorry, you are too low.
// Guess # 5: 84
// You guessed it! What are the odds?!?
// Program Console Sample 2:
// I'm thinking of a number between 1-100. You have 7 guesses.
// First guess: 1
// Sorry, you are too low.
// Guess # 2: 2
// Sorry, you are too low.
// Guess # 3: -8
// Sorry, you are too low.
// Guess # 4: 0
// Sorry, you are too low.
// Guess # 5: 7
// Sorry, you are too low.
// Guess # 6: 612
// Sorry, that guess is too high.
// Guess # 7: -523
// Sorry, you didn't guess it in 7 tries. You lose.


use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("I'm thinking of a number from 1 to 100 . You have 7 guesses");


    for a in 1..8{
        let secret_number = rand::thread_rng().gen_range(1, 101);
        println!("Please input your guess # {}",a);

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("guessed # {} {}",a, guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed it! What are the odds?!?");
                break;
            }
        }
    }
         println!("Sorry, you didn't guess it in 7 tries. You lose.");
}
