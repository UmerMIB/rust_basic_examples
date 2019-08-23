task01
Generate Random Numbers
Generate random floating point and integer numbers with the help of random-number generator rand::Rng obtained
via rand::thread_rng. Note that each thread has an initialized generator.
Note: Integers are uniformly distributed over the range of the type, and floating point numbers are uniformly
distributed from 0 up to but not including 1.
Program Console Sample Output 1:
Random u8: 58
Random u16: 35385
Random u32: 211598899
Random i32: -713122873
Random float: 0.06635079042887482
Program Console Sample Output 2:
Random u8: 229
Random u16: 12600
Random u32: 93015779
Random i32: 1530932671
Random float: 0.925896344335955
Hint for Cargo.toml:
[dependencies]
rand = "0.6.5"
References:
https://crates.io/crates/rand
https://docs.rs/rand/*/rand/trait.Rng.html
https://docs.rs/rand/*/rand/fn.thread_rng.html

use std::io;
use rand::Rng;
fn main(){
    let mut rn_num = rand::thread_rng();
    println!("{}",rn_num.gen::<i32>());
    println!("{}",rn_num.gen::<u16>());
    println!("{}",rn_num.gen::<u32>());
    println!("{}",rn_num.gen::<u8>());
    println!("{}",rn_num.gen::<f32>());
}