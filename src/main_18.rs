task 18
Palindrome tester
Write a program to check whether given input is palindrome or not
Program Console Sample 1:
Enter text: AHA
Text AHA is Palindrome
Program Console Sample 2:
Enter text: Hello
Text Hello is not a Palindrome


fn main() {
    loop
    {
        println!("\nEnter text :");

        let mut text = String::new();
        io::stdin().read_line(&mut text).expect("\nProblem reading data");

        let text_i  = String::from(text.trim());
        let palind_testor = palind(&text_i);

        if palind_testor {
            println!("\nText: {}, is Palindroe",text_i);
        } else {
            println!("\nText: {}, is not a Palindroe",text_i);
        }
        break;
    }
}

fn palind(string: &str) -> bool {
    let half_length = string.len()/2;
    string.chars().take(half_length).eq(string.chars().rev().take(half_length))
}
