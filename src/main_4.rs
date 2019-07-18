Task04
Calculate Volume of a sphere
Write a Rust program to get the volume of a sphere, please take the radius as input from user
Program Console Output:
Enter Radius of Sphere: 1
Volume of the Sphere with Radius 1 is 4.18
Reference:
https://keisan.casio.com/exec/system/1223372883

fn main(){
    
        println!("Please Enter radius of a sphere");
        let Rad :f32 = read!();
           println!("The area of the circle is {}",area(Rad)); 
           
    
}

fn area(rad:f32)-> f32 {
       std::f32::consts::PI*rad*rad*rad*4.0/3.0


}