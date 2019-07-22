task 10
Euclidean distance
Write a Rust program to compute the distance between the points (x1, y1) and (x2, y2).
Program Console Sample 1:
Enter Co-ordinate for x1: 2
Enter Co-ordinate for x2: 4
Enter Co-ordinate for y1: 4
Enter Co-ordinate for y2: 4
Distance between points (2, 4) and (4, 4) is 2
Reference:
https://en.wikipedia.org/wiki/Euclidean_distance
fn main(){
    println!("Enter Co-ordinate for x1: ");
    let  x1 : f32 = read!();
    println!("Enter Co-ordinate for x2: ");
    let  x2 : f32 = read!();
    println!("Enter Co-ordinate for y1: ");
    let  y1 : f32 = read!();
    println!("Enter Co-ordinate for y2: ");
    let  y2 : f32 = read!();
    let mut distance = ((&y1-&x1)*(&y1-&x1))+((&y2-&x2)*(&y2-&x2));

    println!("Distance between points ({}, {}) and ({}, {}) is {}",x1,y1,y2,x2, (distance as f32).sqrt()); 
}
