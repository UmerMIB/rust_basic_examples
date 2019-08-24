// task04
// Sort a Vector of Integers
// Sort a given Vector of integers: [1, 10, 5, 2, 15]
// Program Console Output:
// [1, 2, 5, 10, 15]
// Reference:
// https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort

fn main(){
    let mut vec = [1, 10, 5, 2, 15];
    vec.sort();
   println!("{:?}",vec);
   
}
