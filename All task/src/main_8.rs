task 08
Triangle area
Write a Rust program that will accept the base and height of a triangle and compute the area
Program Console Sample 1:
Enter magnitude of Triangle base: 4
Enter Magnitude of Triangle Height: 4
Area of a Triangle with Height 4 and Base 4 is 8
Reference:https://www.mathgoodies.com/lessons/vol1/area_triangle

fn main(){
    println!("Enter magnitude of Triangle base ");
    let mut base : f32 = read!();
    println!("Enter magnitude of Triangle height ");
    let mut height : f32 = read!();
    
    println!("Area of a Triangle with Height {} and Base {} is {}",&height,&base,area(base,height));
}
fn area(base:f32,height:f32)-> f32{
    (base*height)/2.0
}
