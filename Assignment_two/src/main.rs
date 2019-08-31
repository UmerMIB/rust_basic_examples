#[macro_use]
use std::str;
extern crate text_io;
use std::io;

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// task01
// Generate Random Numbers
// Generate random floating point and integer numbers with the help of random-number generator rand::Rng obtained
// via rand::thread_rng. Note that each thread has an initialized generator.
// Note: Integers are uniformly distributed over the range of the type, and floating point numbers are uniformly
// distributed from 0 up to but not including 1.
// Program Console Sample Output 1:
// Random u8: 58
// Random u16: 35385
// Random u32: 211598899
// Random i32: -713122873
// Random float: 0.06635079042887482
// Program Console Sample Output 2:
// Random u8: 229
// Random u16: 12600
// Random u32: 93015779
// Random i32: 1530932671
// Random float: 0.925896344335955
// Hint for Cargo.toml:
// [dependencies]
// rand = "0.6.5"
// References:
// https://crates.io/crates/rand
// https://docs.rs/rand/*/rand/trait.Rng.html
// https://docs.rs/rand/*/rand/fn.thread_rng.html

// use std::io;
// use rand::Rng;
// fn main(){
//     let mut rn_num = rand::thread_rng();
//     println!("{}",rn_num.gen::<i32>());
//     println!("{}",rn_num.gen::<u16>());
//     println!("{}",rn_num.gen::<u32>());
//     println!("{}",rn_num.gen::<u8>());
//     println!("{}",rn_num.gen::<f32>());
// }
//////////////////////////////////////////////////////////////////////////////////////////////////
// task02
// Generate Random Numbers within a Range
// Generate a random integer and float value within half-open (0, 10) range (not including 10) with Rng::gen_range
// Program Console Sample Output 1:
// Integer: 5
// Float: 2.6670222463244064
// Program Console Sample Output 2:
// Integer: 9
// Float: 1.582827719737141
// use rand::Rng;
// fn main(){
//     let mut num = rand::thread_rng();
//     println!("Integer :{}",num.gen_range(0,10));
//     println!("float :{}", num.gen_range(0.0,10.0));
// } 

//////////////////////////////////////////////////////////////////////////////////////////////////
// task 03
// Generate a Random String
// Randomly generate a string of 30 ASCII characters in the range A-Z, a-z, 0-9, with Alphanumeric sample.
// Program Console Sample Output 1:
// Xc5AHJiPbsTn9qlGkdipTiWrabCpYy
// Program Console Sample Output 2:
// Lwg5Uo2YUEDqOR9m2Qkt8xdAfq133e

// use std::iter;
// use rand::{Rng, thread_rng};
// use rand::distributions::Alphanumeric;
//  fn main(){
//     let mut rng = thread_rng();
//     let chars: String = iter::repeat(())
//             .map(|()| rng.sample(Alphanumeric))
//             .take(30)
//             .collect();
//     println!("Random chars: {}", chars);
//  }


////////////////////////////////////////////////////////////////////////////////////////////////////////
// task04
// Sort a Vector of Integers
// Sort a given Vector of integers: [1, 10, 5, 2, 15]
// Program Console Output:
// [1, 2, 5, 10, 15]
// Reference:
// https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort

// fn main(){
//     let mut vec = [1, 10, 5, 2, 15];
//     vec.sort();
//    println!("{:?}",vec);
   
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////
// task05
// Sort a Vector of Floats
// Sort a given Vector of floats: [1.1, 1.15, 5.5, 1.123, 2.0]
// Program Console Output:
// [1.1, 1.123, 1.15, 2.0, 5.5]
// extern crate quickersort;
// fn main(){
//     let mut vec = [1.1, 1.15, 5.5, 1.123, 2.0];
//     quickersort::sort_floats(&mut vec[..]); 
//    println!("{:?}",vec);
   
// }

//////////////////////////////////////////////////////////////////////////////////////////////////////
// task60
// Generate a Float Vector and Count Elements
// Generate a Vector of 100 random floats between 0 and 900. Find out how many of the random numbers are
// between 0 to 300, 300 to 600, and above 600. 
// use rand::Rng;
// fn main(){
//     let mut vec = Vec::new();
//     for a in 0..100{
//         vec.push(rand::thread_rng().gen_range(0.0,900.0));
//     }
//     quickersort::sort_floats(&mut vec[..]);
    
//     let mut num_a = 0;
//     let mut num_b = 0;
//     let mut num_c = 0;
//     for a in 0.. vec.len(){
//         if vec[a]>0.0 && vec[a]<300.0{
//             num_a += 1;
//         }
//         else if vec[a]>300.0 && vec[a]<600.0{
//             num_b += 1;
//         }
//         else{
//             num_c+= 1;
//         }
//     }
//     println!("Numbers between 0-300 are{}\nNumbers between 300-600 are{} \n Numbers above 600 are{}\n",num_a,num_b,num_c);
// }

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// task07
// Generate a Integer Vector until a certain condition is met
// Generate a random integer vector between 0 to 100 and keep on adding numbers to the vector until a number
// greater than 90 is generated. Once the vector is generated sort it.
// use rand::Rng;
// fn main(){
//     let mut vec = Vec::new();
    
//     loop{
//     let mut a = rand::thread_rng().gen_range(0,101);
//     if a<91{
//         vec.push(a);
//         }
//         else{
//             break;
//         }
//     }
//     vec.sort();
//     println!("{:?}",vec);
// }
///////////////////////////////////////////////////////////////////////////////////////////////////////
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

// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

// fn main() {
//     println!("I'm thinking of a number from 1 to 10");


//     loop {
//         let secret_number = rand::thread_rng().gen_range(1, 11);
//         println!("Please input your guess.");

//         let mut guess = String::new();

//         io::stdin().read_line(&mut guess)
//             .expect("Failed to read line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         println!("You guessed: {}", guess);

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("That's right! My secret number was {}!",&secret_number);
//                 break;
//             }
//         }
//     }
// }


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// task09
// Dice
// Write a program that simulates a dice roll by picking a random number from 1-6 and then picking a second random
// number from 1-6. Add the two values together, and display the total.
// Program Console Sample 1:
// HERE COMES THE DICE!
// Roll #1: 3
// Roll #2: 5
// The total is 8!
// Program Console Sample 2:
// HERE COMES THE DICE!
// Roll #1: 4
// Roll #2: 2
// The total is 6!




// fn main() {
//     println!("HERE COMES THE DICE! ");
//     println!("Roll # 01");
//     let roll_1: i32 = read!();
//     println!("Roll # 02");
//     let roll_2: i32 = read!();
//     println!("The Total is: {}", roll_1+roll_2);
    
// }
///////////////////////////////////////////////////////////////////////////////////////////////////////////
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


// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;
// fn main() {
//     println!("I'm thinking of a number from 1 to 100 . You have 7 guesses");


//     for a in 1..8{
//         let secret_number = rand::thread_rng().gen_range(1, 101);
//         println!("Please input your guess # {}",a);

//         let mut guess = String::new();

//         io::stdin().read_line(&mut guess)
//             .expect("Failed to read line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         println!("guessed # {} {}",a, guess);

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You guessed it! What are the odds?!?");
//                 break;
//             }
//         }
//     }
//          println!("Sorry, you didn't guess it in 7 tries. You lose.");
// }

///////////////////////////////////////////////////////////////////////////////////////////////////
// task11
// Adding Values in a Loop
// Write a program that gets several integers from the user. Sum up all the integers they give you. Stop looping when
// they enter a 0. Display the total at the end.
// Program Console Sample 1:
// I will add up the numbers you give me.
// Number: 6
// The total so far is 6
// Number: 9
// The total so far is 15
// Number: -3
// The total so far is 12
// Number: 2
// The total so far is 14
// Number: 0
// The total is 14.
// Program Console Sample 2:
// I will add up the numbers you give me.
// Number: 1
// The total so far is 1
// Number: 2
// The total so far is 3
// Number: 3
// The total so far is 6
// Number: 4
// The total so far is 10
// Number: 5
// The total so far is 15
// Number: 0
// The total is 15

// fn main(){
//     let mut sum:f32 = 0.0;
//     loop{
//         println!("I will add up the numbers you give me.");
        
//         println!("Number 1 ");
//         let mut inp = String::new();
//         io::stdin()
//             .read_line(&mut inp)
//             .expect("\nProblem reading text");

//         let inp_i: f32 = match inp.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("\nWrong input for radius");
//                 continue;
//             }
//         };
//             sum += inp_i;
//             if inp_i !=0.0{
//                 println!("The total so far is {}",sum);
//             }
//             else if inp_i == 0.0 {
//                 println!("The total so far is {}",sum);
//                 break;
//             }
//     }
// }

// Project Calculator
// Write a calculator program. A minimal calculator will support the following functions:
// • numbers with decimals (not just integers)
// • addition (1 + 2 is 3)
// • subtraction (12 - 4 is 8)
// • multiplication (33 * 2 is 66)
// • division (3 / 8 is 0.375)
// • exponents (2 ^ 3 is 8)
// • error messages when you do something wrong
// Your calculator should keep on running until explicitly told to quit. I suggest typing a zero as the first operand to
// cause it to quit, i.e.
// Program Console Sample:
// >2 + 3
// 5
// >4 * 9
// 36
// >0 + 2
// Bye, now.
// Hint:
// Well, if you read in everything as a String, then you can convert to other things.
// What to avoid:
// Any program, which presents me with a screen like the following, will not receive a very good score.
// Program Console Sample:
// Enter the function you wish to perform.
// 1) addition
// 2) subtraction
// 3) multiplication
// 4) division
// 5) quit
// Your choice:
// Also, the same fate applies to any program that ever presents me with the following message:
// Would you like to calculate again? (y/n) 


// fn main() {
//     let mut flag: char = 'y';

//     loop {
//         if flag == 'n' || flag == 'n' {
//             break;
//         } else if flag == 'y' || flag == 'Y' {
//             println!("Enter The Data To Compute");
//             let data: String = read!();
//             //   println!("{}", data);
//             let mut vec: Vec<char> = Vec::new();
//             let mut count = 0;
//             for x in data.chars() {
//                 //  println!("{}", x);
//                 let y = (x.to_string()).parse::<char>().unwrap();
//                 vec.push(y);
//             }
//             while count < vec.len() {
//                 if vec[count] == '+' {
//                     //  println!("{}", count);
//                     let mut count_two = 0;
//                     let mut count_three = count + 1;
//                     let mut first_value = String::from("");
//                     let mut second_value = String::from("");

//                     while count_two < count {
//                         first_value.push(vec[count_two]);
//                         count_two += 1;
//                     }
//                     while count_three < vec.len() {
//                         second_value.push(vec[count_three]);
//                         count_three += 1;
//                     }
//                     println!(
//                         "The Result Is: {}",
//                         first_value.parse::<i32>().unwrap() + second_value.parse::<i32>().unwrap()
//                     );
//                 }
//                 if vec[count] == '-' {
//                     // println!("{}", count);
//                     let mut count_two = 0;
//                     let mut count_three = count + 1;
//                     let mut first_value = String::from("");
//                     let mut second_value = String::from("");

//                     while count_two < count {
//                         first_value.push(vec[count_two]);
//                         count_two += 1;
//                     }
//                     while count_three < vec.len() {
//                         second_value.push(vec[count_three]);
//                         count_three += 1;
//                     }
//                     println!(
//                         "The Result Is: {}",
//                         first_value.parse::<i32>().unwrap() - second_value.parse::<i32>().unwrap()
//                     );
//                 }
//                 if vec[count] == '/' {
//                     // println!("{}", count);
//                     let mut count_two = 0;
//                     let mut count_three = count + 1;
//                     let mut first_value = String::from("");
//                     let mut second_value = String::from("");

//                     while count_two < count {
//                         first_value.push(vec[count_two]);
//                         count_two += 1;
//                     }
//                     while count_three < vec.len() {
//                         second_value.push(vec[count_three]);
//                         count_three += 1;
//                     }
//                     println!(
//                         "The Result Is: {}",
//                         first_value.parse::<i32>().unwrap() / second_value.parse::<i32>().unwrap()
//                     );
//                 }
//                 if vec[count] == '^' {
//                     // println!("{}", count);
//                     let mut count_two = 0;
//                     let mut count_three = count + 1;
//                     let mut first_value = String::from("");
//                     let mut second_value = String::from("");

//                     while count_two < count {
//                         first_value.push(vec[count_two]);
//                         count_two += 1;
//                     }
//                     while count_three < vec.len() {
//                         second_value.push(vec[count_three]);
//                         count_three += 1;
//                     }
//                     let mut count_pow = 0;
//                     let mut pow_sum = 1;
//                     let fCV = first_value.parse::<i32>().unwrap();
//                     let sCV = second_value.parse::<i32>().unwrap();
//                     while count_pow < sCV {
//                         pow_sum *= fCV;
//                         count_pow += 1;
//                     }
//                     println!("The Result Is: {}", pow_sum);
//                 }
//                 count += 1;
//             }
//         } else {
//             println!("Invalid Selection!");
//         }
//         println!("Do you want to continue program y/n ?");
//         flag = read!();
//     }
// }