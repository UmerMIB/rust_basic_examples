// task05
// Sort a Vector of Floats
// Sort a given Vector of floats: [1.1, 1.15, 5.5, 1.123, 2.0]
// Program Console Output:
// [1.1, 1.123, 1.15, 2.0, 5.5]

extern crate quickersort;
fn main(){
    let mut vec = [1.1, 1.15, 5.5, 1.123, 2.0];
    quickersort::sort_floats(&mut vec[..]); 
   println!("{:?}",vec);
   
}