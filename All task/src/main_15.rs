task15
Decimal to Binary Converter
Write a Rust program to convert an decimal integer to binary
Program Console Sample 1:
Enter a decimal number: 5
Binary Representation of 5 is 101
Program Console Sample 2:
Enter a decimal number: 32
Binary Representation of 32 is 100000
Reference:
https://www.rapidtables.com/convert/number/decimal-to-binary.html

fn main(){
     println!("Enter a decimal number: ");
     let  n: u32 = read!();
     println!("Binary Representation of {} is {:b}",n,n);
}
