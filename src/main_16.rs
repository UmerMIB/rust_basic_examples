task16
Binary to Decimal Converter
Write a program to convert binary number to Decimal number
Program Console Sample 1:
Enter a Binary number: 1101
Decimal Representation of 1101 is 13
Program Console Sample 2:
Enter a Binary number: 1001
Decimal Representation of 1001 is 9
Reference:
https://www.rapidtables.com/convert/number/binary-to-decimal.html
https://stackoverflow.com/questions/27606616/convert-string-with-binary-to-int

fn main(){
     println!("Enter a binary number: ");
     let  mut n = String::new();
     io::stdin().read_line(&mut n).expect("Error reading file");
     let n_i = String::from(n.trim()); 
     let intval = isize::from_str_radix(&n_i, 2).unwrap();
    println!("{}", intval);
}
