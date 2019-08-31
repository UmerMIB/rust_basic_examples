task20
Write a Rust program to construct the following pattern
*
* *
* * *
* * * *
* * * * *
* * * *
* * *
* *
*
fn main(){
     for _a in 1..7{
          for _b in 1.._a{
          print!("*");
          }
          println!("");
     }
         
     for _a in (1..6).rev(){
          for _b in (1.._a).rev(){
          print!("*");
          }
          println!("");
     }
}