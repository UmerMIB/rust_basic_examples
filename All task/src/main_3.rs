Task 03
Divisibility Check of two numbers
Write a Rust program to check whether a number is completely divisible by another number. Accept two integer
values form the user
Program Console Sample Output 1:
Enter numerator: 4
Enter Denominator: 2
Number 4 is Completely divisible by 2
Program Console Sample Output 2:
Enter numerator: 7
Enter Denominator: 4
Number 7 is not Completely divisible by 4


fn main(){
    println!("Please enter a Numerator");
    let mut num : f32 = read!();
    println!("Please enter a Denomerator");
    let mut dem : f32 = read!();
    println!("{}", division(num,dem));
    
}
fn division(x:f32, y:f32)->String{
    if x%y == 0.0{
        String::from("Divisible numbers")
    }
    else{ 
        String::from("Indivisible numbers")
    }
    
}
