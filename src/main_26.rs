task 03
Generate a Random String
Randomly generate a string of 30 ASCII characters in the range A-Z, a-z, 0-9, with Alphanumeric sample.
Program Console Sample Output 1:
Xc5AHJiPbsTn9qlGkdipTiWrabCpYy
Program Console Sample Output 2:
Lwg5Uo2YUEDqOR9m2Qkt8xdAfq133e

use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
 fn main(){
    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(30)
            .collect();
    println!("Random chars: {}", chars);
 }