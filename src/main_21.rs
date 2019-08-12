task21
Write a Rust program to construct the following pattern
1
1 2
1 2 3
1 2 3 4
1 2 3 4 5
1 2 3 4
1 2 3
1 2
1

fn main(){
     for _a in 1..7{
          for _b in 1.._a{
          print!("{}",_b);
          }
          println!("");
     }
         
     for _a in (1..6).rev(){
          for _b in (1.._a).rev(){
          print!("{}",_b);
          }
          println!("");
     }
}
