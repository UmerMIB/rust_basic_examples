task11
Write a Rust program to convert height in feet to centimetres.
Program Console Sample 1:
Enter Height in Feet: 5
There are 152.4 Cm in 5 ft
Reference:
https://www.rapidtables.com/convert/length/feet-to-cm.html
 
 fn main(){
    println!("Enter Height in Feet: ");
    let  height_in_feet : f32 = read!();
    let  height_in_cm : f32 = height_in_feet * 30.48;
    println!("There are {} Cm in {} feet",height_in_cm,height_in_feet );
 }
