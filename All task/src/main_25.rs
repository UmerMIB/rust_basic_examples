task02
Generate Random Numbers within a Range
Generate a random integer and float value within half-open (0, 10) range (not including 10) with Rng::gen_range
Program Console Sample Output 1:
Integer: 5
Float: 2.6670222463244064
Program Console Sample Output 2:
Integer: 9
Float: 1.582827719737141
use rand::Rng;
fn main(){
    let mut num = rand::thread_rng();
    println!("Integer :{}",num.gen_range(0,10));
    println!("float :{}", num.gen_range(0.0,10.0));
} 