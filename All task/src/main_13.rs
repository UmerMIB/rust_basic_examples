task 13
Sum of n Positive Integers
Write a Rust program to sum of the first n positive integers
Program Console Sample 1:
Enter value of n: 5
Sum of n Positive integers till 5 is 15

fn main(){
     println!("Enter value of n: ");
     let  n: u32 = read!();
     println!("Sum of n Positive integers till {} is {}", n, sum(n));
}
fn sum(n:u32) -> u32{
    let mut inc :u32 = 0;
    for a in 0..(n+1){
        inc = inc + a
     }
     inc
}
