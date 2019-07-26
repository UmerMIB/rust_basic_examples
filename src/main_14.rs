task 14
 Digits Sum of a Number
Write a Rust program to calculate the sum of the digits in an integer
Program Console Sample 1:
Enter a number: 15
Sum of 1 + 5 is 6
Program Console Sample 2:
Enter a number: 1234
Sum of 1 + 2 + 3 + 4 is 10
fn main(){
    println!("Enter a number:");
    let mut n = String::new();
    io::stdin().read_line(&mut n)
    .expect("Error reading file");
    let mut n_i = String::from(n.trim());
    let mut sum :u32 = 0;
    for a in 0..n_i.len(){
        let mut temp = &n_i[a..(a+1)];
        let mut temp_i :u32 = match temp.trim().parse(){
            Ok(num)=>num,
            Err(_) => {
                println!("\nWrong input");
                continue;
            }
        };
        sum += temp_i;
        
        println!("{}",temp_i);
        if a == n_i.len() -1{
            println!("sum is {}", sum)
        }
        else{
            println!{"+"}
        }
    }

}
