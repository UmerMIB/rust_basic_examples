task 01
Calculate Area of a Circle
Write a Rust program, whiv accepts the radius of a circle from the user and computes the area.
Program Console Sample Output 1:
Input Radius: 0.5
Area of Circle with radius 0.5 is 0.7853981634
References:
https://www.mathsisfun.com/geometry/circle-area.html



fn main(){
    
        println!("Please enter radius of the circle");
        let Rad:f32 = read!();
           println!("The area of the circle is {}",area(Rad)); 
          
    
}
      fn area(rad:f32)-> f32 {
       std::f32::consts::PI*rad*rad
}