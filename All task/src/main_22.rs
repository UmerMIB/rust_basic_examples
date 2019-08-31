task 22
Write a Rust program to construct the following pattern
1
22
333
4444
55555
666666
7777777
88888888
999999999

fn main(){
     for _a in 0..10{
          for _b in 0.._a{
          print!("{}",_a);
          }
          println!("");
     }
         
}