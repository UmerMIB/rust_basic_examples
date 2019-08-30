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




fn main() {
    println!("HERE COMES THE DICE! ");
    println!("Roll # 01");
    let roll_1: i32 = read!();
    println!("Roll # 02");
    let roll_2: i32 = read!();
    println!("The Total is: {}", roll_1+roll_2);
    
}