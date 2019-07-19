task 05
Copy string n times
Write a Rust program to get a string which is n (non-negative integer) copies of a given string.
Program Console Output:
Enter String: Hi
How many copies of String you need: 4
4 Copies of Hi are HiHiHiHi


fn main(){
           println!("Please Enter something");
        let mut string  = String::new();
        io::stdin().read_line(&mut string).expect("Error reading file");
           println!("Please Enter how many times it would be print");
           let n :u16 = read!();
           println!("{}",prints(n,&string));

}
fn prints(c: u16, string: &String) -> String {
    let mut p = String::new();
    for a in 0..c {
       p = p + &string;
    }
    p
}
