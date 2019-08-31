task 06
Check if number is Even or Odd
Write a Rust program to find whether a given number (accept from the user) is even or odd, print out an
appropriate message to the user
Program Console Output 1:
Enter Number: 4
4 is Even
Program Console Output 2:
Enter Number: 9
9 is Odd

fn main(){
    println!("Please enter an integer");
    let mut num : i32 = read!();
    println!("{} is {}",num,Even_Odd(num));
}
fn Even_Odd(n:i32)-> String{
    if n%2 == 0{
        String::from("Even")
    }
    else{
        String::from("Odd")
    }
}
