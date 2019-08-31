Task07
Vowel Tester
Write a Rust program to test whether a passed letter is a vowel or not
Program Console Output 1:
Enter a character: A
Letter A is Vowel
Program Console Output 2:
Enter a character: e
Letter e is Vowel
Program Console Output 2:
Enter a character: N
Letter N is not Vowel

fn main(){
    println!("Please Enter 1 letter");
    let mut string  = String::new();
    io::stdin().read_line(&mut string).expect("Error reading file");
    let chr = String::from(string.trim());
    if chr.len()>1{
        println!("Please only 1 character")
    }
    println!("{}",vowel(chr));

}
fn vowel(v:String)-> String{
    if v == "a" || v == "A" || v == "e" || v == "E" || v == "i" || v == "I" || v == "o" || v == "O" || v == "u" || v == "U"
    {
        String::from("Vowel")
    } else
    {
        String::from("not Vowel")
    }

}
