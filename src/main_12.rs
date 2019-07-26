Task 12
BMI Calculator
Write a Rust program to calculate body mass index
Program Console Sample 1:
Enter Height in Cm: 180
Enter Weight in Kg: 75
Your BMI is 23.15
Reference:
https://www.thecalculatorsite.com/articles/health/bmi-formula-for-bmi-calculations.php


fn main(){
     println!("Enter Height in Cm: ");
     let  height_in_Cm : f32 = read!();
     let  height_in_m : f32 = height_in_Cm/100.0;
     println!("Enter Weight in Kg: ");
     let weight : f32 = read!();
     let BMI = (weight)/(height_in_m*height_in_m);
     println!("Your BMI is {}", BMI);
}
