task02
Check Number either positive, negative or zero
Write a Rust program to check if a number is positive, negative or zero
Program Console Sample Output 1:
Enter Number: -1
Negative Number Entered
Program Console Sample Output 2:
Integer: 3
Positive Number Entered
Program Console Sample Output 3:
Integer: 0
Zero Entered


fn main(){
    println!("Please enter a number");
    let mut input : f32 = read!();
    println!("{} number entered",check(input));
}
fn check (n:f32)->String{
    if n<0.0{
        String::from("Negative")
    }
    else if n>0.0{
        String::from("Positive")
    }
    else{
    String::from("Zero")
    }
}