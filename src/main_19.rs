task 19
. Count Alphabets, Numbers and Special Characters
Write a Rust program that accepts a string and calculate the number of digits and letters
Program Console Sample 1:
Enter text: Rust 3.2
Numbers = 2
Alphabets = 6
Special Characters = 1
Spaces = 1

fn main(){
     println!("Enter something");
      let mut i = String::new();
        io::stdin()
            .read_line(&mut i)
            .expect("\nProblem reading data");

        let i_text = String::from(i.trim());
        count(&i_text);

}
fn count(string: &String){
     let mut alpha = 0;
     let mut numbers = 0;
     let mut spchr = 0;
     let mut spaces = 0;
     for a in string.chars(){
          match a{
               'a'...'z' => alpha   += 1,
               'A'...'Z' => alpha   += 1,
               '0'...'9' => numbers += 1,
               ' ' => spaces += 1,
               _ => spchr +=  1,
          }
     }
          println!("\nNumber = {}", numbers);
          println!("\nAlphabets = {}", alpha);
          println!("\nSpecial Characters = {}", spchr);
          println!("\nSpaces = {}", spaces);
}
