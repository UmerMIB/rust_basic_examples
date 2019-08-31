
#[macro_use]
extern crate text_io;
use std::str;
use std::io;
/////////////////////////////////////////////////////////////////////////
//task 01
//Calculate Area of a Circle
// Write a Rust program, whiv accepts the radius of a circle from the user and computes the area.
// Program Console Sample Output 1:
// Input Radius: 0.5
// Area of Circle with radius 0.5 is 0.7853981634
// References:
// https://www.mathsisfun.com/geometry/circle-area.html



// fn main(){
//     
//         println!("Please enter radius of the circle");
//         let mut rad = String::new();
//         io::stdin().read_line(&mut rad).expect("Error reading file");
//         let mut Rad :f32 = match rad.trim().parse(){
//             Ok(num) => num,
//             Err(_) => {
//                 println!("Please Enter a number ");
//                 continue;
//             }
//         };
//            println!("The area of the circle is {}",area(Rad)); 
//           
//     
// }
//       fn area(rad:f32)-> f32 {
//        std::f32::consts::PI*rad*rad
// }
////////////////////////////////////////////////////////////////////////////////////////////
//task02
// Check Number either positive, negative or zero
// Write a Rust program to check if a number is positive, negative or zero
// Program Console Sample Output 1:
// Enter Number: -1
// Negative Number Entered
// Program Console Sample Output 2:
// Integer: 3
// Positive Number Entered
// Program Console Sample Output 3:
// Integer: 0
// Zero Entered


// fn main(){
//     println!("Please enter a number");
//     let mut input : f32 = read!();
//     println!("{} number entered",check(input));
// }
// fn check (n:f32)->String{
//     if n<0.0{
//         String::from("Negative")
//     }
//     else if n>0.0{
//         String::from("Positive")
//     }
//     else{
//     String::from("Zero")
//     }
// }
//////////////////////////////////////////////////////////////////////////////////////////////
//Task 03
// Divisibility Check of two numbers
// Write a Rust program to check whether a number is completely divisible by another number. Accept two integer
// values form the user
// Program Console Sample Output 1:
// Enter numerator: 4
// Enter Denominator: 2
// Number 4 is Completely divisible by 2
// Program Console Sample Output 2:
// Enter numerator: 7
// Enter Denominator: 4
// Number 7 is not Completely divisible by 4


// fn main(){
//     println!("Please enter a Numerator");
//     let mut num : f32 = read!();
//     println!("Please enter a Denomerator");
//     let mut dem : f32 = read!();
//     println!("{}", division(num,dem));
    
// }
// fn division(x:f32, y:f32)->String{
//     if x%y == 0.0{
//         String::from("Divisible numbers")
//     }
//     else{ 
//         String::from("Indivisible numbers")
//     }
    
// }

/////////////////////////////////////////////////////////////////////////////////////////////
//Task04
// Calculate Volume of a sphere
// Write a Rust program to get the volume of a sphere, please take the radius as input from user
// Program Console Output:
// Enter Radius of Sphere: 1
// Volume of the Sphere with Radius 1 is 4.18
// Reference:
// https://keisan.casio.com/exec/system/1223372883

// fn main(){
//     
//         println!("Please Enter radius of a sphere");
//         let mut rad = String::new();
//         io::stdin().read_line(&mut rad).expect("Error reading file");
//         let mut Rad :f32 = match rad.trim().parse(){
//             Ok(num) => num,
//             Err(_) => {
//                 println!("Please Enter a number ");
//                 continue;
//             }
//         };
//            println!("The area of the circle is {}",area(Rad)); 
//            
//     
// }

// fn area(rad:f32)-> f32 {
//        std::f32::consts::PI*rad*rad*rad*4.0/3.0


// }

///////////////////////////////////////////////////////////////////////////
// task 05
// Copy string n times
// Write a Rust program to get a string which is n (non-negative integer) copies of a given string.
// Program Console Output:
// Enter String: Hi
// How many copies of String you need: 4
// 4 Copies of Hi are HiHiHiHi


// fn main(){
//            println!("Please Enter something");
//         let mut string  = String::new();
//         io::stdin().read_line(&mut string).expect("Error reading file");
//            println!("Please Enter how many times it would be print");
//            let n :u16 = read!();
//            println!("{}",prints(n,&string));

// }
// fn prints(c: u16, string: &String) -> String {
//     let mut p = String::new();
//     for a in 0..c {
//        p = p + &string;
//     }
//     p
// }

////////////////////////////////////////////////////////////////////////////////////////////////////
// task 06
// Check if number is Even or Odd
// Write a Rust program to find whether a given number (accept from the user) is even or odd, print out an
// appropriate message to the user
// Program Console Output 1:
// Enter Number: 4
// 4 is Even
// Program Console Output 2:
// Enter Number: 9
// 9 is Odd

// fn main(){
//     println!("Please enter an integer");
//     let mut num : i32 = read!();
//     println!("{} is {}",num,Even_Odd(num));
// }
// fn Even_Odd(n:i32)-> String{
//     if n%2 == 0{
//         String::from("Even")
//     }
//     else{
//         String::from("Odd")
//     }
// }

/////////////////////////////////////////////////////////////////////////////////////////////
// Task07
// Vowel Tester
// Write a Rust program to test whether a passed letter is a vowel or not
// Program Console Output 1:
// Enter a character: A
// Letter A is Vowel
// Program Console Output 2:
// Enter a character: e
// Letter e is Vowel
// Program Console Output 2:
// Enter a character: N
// Letter N is not Vowel

// fn main(){
//     println!("Please Enter 1 letter");
//     let mut string  = String::new();
//     io::stdin().read_line(&mut string).expect("Error reading file");
//     let chr = String::from(string.trim());
//     if chr.len()>1{
//         println!("Please only 1 character")
//     }
//     println!("{}",vowel(chr));

// }
// fn vowel(v:String)-> String{
//     if v == "a" || v == "A" || v == "e" || v == "E" || v == "i" || v == "I" || v == "o" || v == "O" || v == "u" || v == "U"
//     {
//         String::from("Vowel")
//     } else
//     {
//         String::from("not Vowel")
//     }

// }

///////////////////////////////////////////////////////////////////////////////////////////////////////////
// task 08
// Triangle area
// Write a Rust program that will accept the base and height of a triangle and compute the area
// Program Console Sample 1:
// Enter magnitude of Triangle base: 4
// Enter Magnitude of Triangle Height: 4
// Area of a Triangle with Height 4 and Base 4 is 8
// Reference:https://www.mathgoodies.com/lessons/vol1/area_triangle

// fn main(){
//     println!("Enter magnitude of Triangle base ");
//     let mut base : f32 = read!();
//     println!("Enter magnitude of Triangle height ");
//     let mut height : f32 = read!();
    
//     println!("Area of a Triangle with Height {} and Base {} is {}",&height,&base,area(base,height));
// }
// fn area(base:f32,height:f32)-> f32{
//     (base*height)/2.0
// }

////////////////////////////////////////////////////////////////////////////////////////////////////////
// task 09
// Calculate Interest
// Write a Rust program to compute the future value of a specified principal amount, rate of interest, and a number of
// years
// Program Console Sample 1:
// Please enter principal amount: 10000
// Please Enter Rate of interest in %: 0.1
// Enter number of years for investment: 5
// After 5 years your principal amount 10000 over an interest rate of 0.1 % will be 16105.1

// fn main(){
//     println!("Please enter principal amount: ");
//     let mut pm : f32 = read!();
//     println!("Please Enter Rate of interest in %: ");
//     let mut ri : f32 = read!();
//     println!("Enter number of years for investment: ");
//     let mut yrs : f32 = read!();
//     println!("After {} years your principal amount {} over an interest rate of {}% will be {} ",&yrs,&pm,&ri,amount(pm,ri,yrs));
// }
// fn amount(pm:f32,ri:f32,yrs:f32)->f32{
//     pm*(1.0+(ri*yrs))
// }

////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// task 10
// Euclidean distance
// Write a Rust program to compute the distance between the points (x1, y1) and (x2, y2).
// Program Console Sample 1:
// Enter Co-ordinate for x1: 2
// Enter Co-ordinate for x2: 4
// Enter Co-ordinate for y1: 4
// Enter Co-ordinate for y2: 4
// Distance between points (2, 4) and (4, 4) is 2
// Reference:
// https://en.wikipedia.org/wiki/Euclidean_distance
// fn main(){
//     println!("Enter Co-ordinate for x1: ");
//     let  x1 : f32 = read!();
//     println!("Enter Co-ordinate for x2: ");
//     let  x2 : f32 = read!();
//     println!("Enter Co-ordinate for y1: ");
//     let  y1 : f32 = read!();
//     println!("Enter Co-ordinate for y2: ");
//     let  y2 : f32 = read!();
//     let mut distance = ((&y1-&x1)*(&y1-&x1))+((&y2-&x2)*(&y2-&x2));

//     println!("Distance between points ({}, {}) and ({}, {}) is {}",x1,y1,y2,x2, (distance as f32).sqrt()); 
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// task11
// Write a Rust program to convert height in feet to centimetres.
// Program Console Sample 1:
// Enter Height in Feet: 5
// There are 152.4 Cm in 5 ft
// Reference:
// https://www.rapidtables.com/convert/length/feet-to-cm.html
 
//  fn main(){
//     println!("Enter Height in Feet: ");
//     let  height_in_feet : f32 = read!();
//     let  height_in_cm : f32 = height_in_feet * 30.48;
//     println!("There are {} Cm in {} feet",height_in_cm,height_in_feet );
//  }

////////////////////////////////////////////////////////////////////////////////////////////////
// Task 12
// BMI Calculator
// Write a Rust program to calculate body mass index
// Program Console Sample 1:
// Enter Height in Cm: 180
// Enter Weight in Kg: 75
// Your BMI is 23.15
// Reference:
// https://www.thecalculatorsite.com/articles/health/bmi-formula-for-bmi-calculations.php


// fn main(){
//      println!("Enter Height in Cm: ");
//      let  height_in_Cm : f32 = read!();
//      let  height_in_m : f32 = height_in_Cm/100.0;
//      println!("Enter Weight in Kg: ");
//      let weight : f32 = read!();
//      let BMI = (weight)/(height_in_m*height_in_m);
//      println!("Your BMI is {}", BMI);
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////
//task 13
// Sum of n Positive Integers
// Write a Rust program to sum of the first n positive integers
// Program Console Sample 1:
// Enter value of n: 5
// Sum of n Positive integers till 5 is 15

// fn main(){
//      println!("Enter value of n: ");
//      let  n: u32 = read!();
//      println!("Sum of n Positive integers till {} is {}", n, sum(n));
// }
// fn sum(n:u32) -> u32{
//     let mut inc :u32 = 0;
//     for a in 0..(n+1){
//         inc = inc + a
//      }
//      inc
// }

////////////////////////////////////////////////////////////////////////////////////////////////////////
// task 14
//  Digits Sum of a Number
// Write a Rust program to calculate the sum of the digits in an integer
// Program Console Sample 1:
// Enter a number: 15
// Sum of 1 + 5 is 6
// Program Console Sample 2:
// Enter a number: 1234
// Sum of 1 + 2 + 3 + 4 is 10
// fn main(){
//     println!("Enter a number:");
//     let mut n = String::new();
//     io::stdin().read_line(&mut n)
//     .expect("Error reading file");
//     let mut n_i = String::from(n.trim());
//     let mut sum :u32 = 0;
//     for a in 0..n_i.len(){
//         let mut temp = &n_i[a..(a+1)];
//         let mut temp_i :u32 = match temp.trim().parse(){
//             Ok(num)=>num,
//             Err(_) => {
//                 println!("\nWrong input");
//                 continue;
//             }
//         };
//         sum += temp_i;
        
//         println!("{}",temp_i);
//         if a == n_i.len() -1{
//             println!("sum is {}", sum)
//         }
//         else{
//             println!{"+"}
//         }
//     }

// }

///////////////////////////////////////////////////////////////////////////////////////////////////
// task15
// Decimal to Binary Converter
// Write a Rust program to convert an decimal integer to binary
// Program Console Sample 1:
// Enter a decimal number: 5
// Binary Representation of 5 is 101
// Program Console Sample 2:
// Enter a decimal number: 32
// Binary Representation of 32 is 100000
// Reference:
// https://www.rapidtables.com/convert/number/decimal-to-binary.html

// fn main(){
//      println!("Enter a decimal number: ");
//      let  n: u32 = read!();
//      println!("Binary Representation of {} is {:b}",n,n);
// }

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
// task16
// Binary to Decimal Converter
// Write a program to convert binary number to Decimal number
// Program Console Sample 1:
// Enter a Binary number: 1101
// Decimal Representation of 1101 is 13
// Program Console Sample 2:
// Enter a Binary number: 1001
// Decimal Representation of 1001 is 9
// Reference:
// https://www.rapidtables.com/convert/number/binary-to-decimal.html
//https://stackoverflow.com/questions/27606616/convert-string-with-binary-to-int

// fn main(){
//      println!("Enter a binary number: ");
//      let  mut n = String::new();
//      io::stdin().read_line(&mut n).expect("Error reading file");
//      let n_i = String::from(n.trim()); 
//      let intval = isize::from_str_radix(&n_i, 2).unwrap();
//     println!("{}", intval);
// }

////////////////////////////////////////////////////////////////////////////////////////////////////
// task17
// Vowel and Consonants Counter
// Input a text and count the occurrences of vowels and consonant
// Program Console Sample 1:
// Enter text: QuickBrownFoxJumpsovertheDog
// Vowels: 9
// Consonants: 19


// fn main() {
//     println!("please enter something");
//     let string: String = read!();
//     //   println!("{}", data);
//     let mut vowel: Vec<char> = Vec::new();
//     let mut count = 0;
//     let mut Vowel_Count = 0;
//     let mut Consonant_Count = 0;
//     for x in string.chars() {
//         let y = (x.to_string()).parse::<char>().unwrap();
//         vowel.push(y);
//     }
//     while count < vowel.len() {
//         if vowel[count] == 'a'
//             || vowel[count] == 'e'
//             || vowel[count] == 'i'
//             || vowel[count] == 'o'
//             || vowel[count] == 'u'
//         {
//             Vowel_Count += 1;
//         } else {
//             Consonant_Count += 1;
//         }  
//         count +=1;
//     }
//      println!("VOWELS: {} Consonent: {}", Vowel_Count, Consonant_Count);
// }

//////////////////////////////////////////////////////////////////////////////////////////////////
// task 18
// Palindrome tester
// Write a program to check whether given input is palindrome or not
// Program Console Sample 1:
// Enter text: AHA
// Text AHA is Palindrome
// Program Console Sample 2:
// Enter text: Hello
// Text Hello is not a Palindrome


// fn main() {
//     loop
//     {
//         println!("\nEnter text :");

//         let mut text = String::new();
//         io::stdin().read_line(&mut text).expect("\nProblem reading data");

//         let text_i  = String::from(text.trim());
//         let palind_testor = palind(&text_i);

//         if palind_testor {
//             println!("\nText: {}, is Palindroe",text_i);
//         } else {
//             println!("\nText: {}, is not a Palindroe",text_i);
//         }
//         break;
//     }
// }

// fn palind(string: &str) -> bool {
//     let half_length = string.len()/2;
//     string.chars().take(half_length).eq(string.chars().rev().take(half_length))
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////
// task 19
// . Count Alphabets, Numbers and Special Characters
// Write a Rust program that accepts a string and calculate the number of digits and letters
// Program Console Sample 1:
// Enter text: Rust 3.2
// Numbers = 2
// Alphabets = 6
// Special Characters = 1
// Spaces = 1

// fn main(){
//      println!("Enter something");
//       let mut i = String::new();
//         io::stdin()
//             .read_line(&mut i)
//             .expect("\nProblem reading data");

//         let i_text = String::from(i.trim());
//         count(&i_text);

// }
// fn count(string: &String){
//      let mut alpha = 0;
//      let mut numbers = 0;
//      let mut spchr = 0;
//      let mut spaces = 0;
//      for a in string.chars(){
//           match a{
//                'a'...'z' => alpha   += 1,
//                'A'...'Z' => alpha   += 1,
//                '0'...'9' => numbers += 1,
//                ' ' => spaces += 1,
//                _ => spchr +=  1,
//           }
//      }
//           println!("\nNumber = {}", numbers);
//           println!("\nAlphabets = {}", alpha);
//           println!("\nSpecial Characters = {}", spchr);
//           println!("\nSpaces = {}", spaces);
// }


///////////////////////////////////////////////////////////////////////////////////////////////////////////
// task20
// Write a Rust program to construct the following pattern
// *
// * *
// * * *
// * * * *
// * * * * *
// * * * *
// * * *
// * *
// *
// fn main(){
//      for _a in 1..7{
//           for _b in 1.._a{
//           print!("*");
//           }
//           println!("");
//      }
         
//      for _a in (1..6).rev(){
//           for _b in (1.._a).rev(){
//           print!("*");
//           }
//           println!("");
//      }
// }
//////////////////////////////////////////////////////////////////////////////////////////////////////
// task21
// Write a Rust program to construct the following pattern
// 1
// 1 2
// 1 2 3
// 1 2 3 4
// 1 2 3 4 5
// 1 2 3 4
// 1 2 3
// 1 2
// 1

// fn main(){
//      for _a in 1..7{
//           for _b in 1.._a{
//           print!("{}",_b);
//           }
//           println!("");
//      }
         
//      for _a in (1..6).rev(){
//           for _b in (1.._a).rev(){
//           print!("{}",_b);
//           }
//           println!("");
//      }
// }

///////////////////////////////////////////////////////////////////////////////////////////////
// task 22
// Write a Rust program to construct the following pattern
// 1
// 22
// 333
// 4444
// 55555
// 666666
// 7777777
// 88888888
// 999999999

// fn main(){
//      for _a in 0..10{
//           for _b in 0.._a{
//           print!("{}",_a);
//           }
//           println!("");
//      }
         
// }

///////////////////////////////////////////////////////////////////////////////////////////////////
// task23
// Days Calculator
// Write a Rust program to calculate number of days between two dates
// Program Console Output:
// Enter a date in (dd/mm/yy) format: 12/12/2018
// Enter a date in (dd/mm/yy) format: 16/12/2018
// There are 4 days in between 12/12/2018 and 16/12/18
// Reference:
// https://crates.io/crates/chrono

// use chrono::{ NaiveDate};
// use chrono::format::ParseError;

// fn main() -> Result<(), ParseError> {
//     println!("To get the difference between the dates please");
//     let mut date1 = String::new();

//     println!("Enter a date in (format 2015-09-05 YYY-MM-DD) ");
//     io::stdin().read_line(&mut date1).expect("\nProblem reading data");

//     let date_1 = NaiveDate::parse_from_str(&date1.trim(), "%Y-%m-%d")?;

//     let mut date2 = String::new();

//     println!("Enter a date in (format 2015-09-05 YYY-MM-DD) ");
//     io::stdin().read_line(&mut date2).expect("\nProblem reading data");

//     let date_2 = NaiveDate::parse_from_str(&date2.trim(), "%Y-%m-%d")?;

//     let duration = date_2.signed_duration_since(date_1);

//     println!("There are {:?} days in between {} and {}",duration.num_days(),date_1,date_2);

//     Ok(())
// }
