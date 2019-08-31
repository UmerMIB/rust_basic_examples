task 09
Calculate Interest
Write a Rust program to compute the future value of a specified principal amount, rate of interest, and a number of
years
Program Console Sample 1:
Please enter principal amount: 10000
Please Enter Rate of interest in %: 0.1
Enter number of years for investment: 5
After 5 years your principal amount 10000 over an interest rate of 0.1 % will be 16105.1

fn main(){
    println!("Please enter principal amount: ");
    let mut pm : f32 = read!();
    println!("Please Enter Rate of interest in %: ");
    let mut ri : f32 = read!();
    println!("Enter number of years for investment: ");
    let mut yrs : f32 = read!();
    println!("After {} years your principal amount {} over an interest rate of {}% will be {} ",&yrs,&pm,&ri,amount(pm,ri,yrs));
}
fn amount(pm:f32,ri:f32,yrs:f32)->f32{
    pm*(1.0+(ri*yrs))
}
