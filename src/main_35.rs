// Project Calculator
// Write a calculator program. A minimal calculator will support the following functions:
// • numbers with decimals (not just integers)
// • addition (1 + 2 is 3)
// • subtraction (12 - 4 is 8)
// • multiplication (33 * 2 is 66)
// • division (3 / 8 is 0.375)
// • exponents (2 ^ 3 is 8)
// • error messages when you do something wrong
// Your calculator should keep on running until explicitly told to quit. I suggest typing a zero as the first operand to
// cause it to quit, i.e.
// Program Console Sample:
// >2 + 3
// 5
// >4 * 9
// 36
// >0 + 2
// Bye, now.
// Hint:
// Well, if you read in everything as a String, then you can convert to other things.
// What to avoid:
// Any program, which presents me with a screen like the following, will not receive a very good score.
// Program Console Sample:
// Enter the function you wish to perform.
// 1) addition
// 2) subtraction
// 3) multiplication
// 4) division
// 5) quit
// Your choice:
// Also, the same fate applies to any program that ever presents me with the following message:
// Would you like to calculate again? (y/n) 


fn main() {
    let mut flag: char = 'y';

    loop {
        if flag == 'n' || flag == 'n' {
            break;
        } else if flag == 'y' || flag == 'Y' {
            println!("Enter The Data To Compute");
            let data: String = read!();
            //   println!("{}", data);
            let mut vec: Vec<char> = Vec::new();
            let mut count = 0;
            for x in data.chars() {
                //  println!("{}", x);
                let y = (x.to_string()).parse::<char>().unwrap();
                vec.push(y);
            }
            while count < vec.len() {
                if vec[count] == '+' {
                    //  println!("{}", count);
                    let mut count_two = 0;
                    let mut count_three = count + 1;
                    let mut first_value = String::from("");
                    let mut second_value = String::from("");

                    while count_two < count {
                        first_value.push(vec[count_two]);
                        count_two += 1;
                    }
                    while count_three < vec.len() {
                        second_value.push(vec[count_three]);
                        count_three += 1;
                    }
                    println!(
                        "The Result Is: {}",
                        first_value.parse::<i32>().unwrap() + second_value.parse::<i32>().unwrap()
                    );
                }
                if vec[count] == '-' {
                    // println!("{}", count);
                    let mut count_two = 0;
                    let mut count_three = count + 1;
                    let mut first_value = String::from("");
                    let mut second_value = String::from("");

                    while count_two < count {
                        first_value.push(vec[count_two]);
                        count_two += 1;
                    }
                    while count_three < vec.len() {
                        second_value.push(vec[count_three]);
                        count_three += 1;
                    }
                    println!(
                        "The Result Is: {}",
                        first_value.parse::<i32>().unwrap() - second_value.parse::<i32>().unwrap()
                    );
                }
                if vec[count] == '/' {
                    // println!("{}", count);
                    let mut count_two = 0;
                    let mut count_three = count + 1;
                    let mut first_value = String::from("");
                    let mut second_value = String::from("");

                    while count_two < count {
                        first_value.push(vec[count_two]);
                        count_two += 1;
                    }
                    while count_three < vec.len() {
                        second_value.push(vec[count_three]);
                        count_three += 1;
                    }
                    println!(
                        "The Result Is: {}",
                        first_value.parse::<i32>().unwrap() / second_value.parse::<i32>().unwrap()
                    );
                }
                if vec[count] == '^' {
                    // println!("{}", count);
                    let mut count_two = 0;
                    let mut count_three = count + 1;
                    let mut first_value = String::from("");
                    let mut second_value = String::from("");

                    while count_two < count {
                        first_value.push(vec[count_two]);
                        count_two += 1;
                    }
                    while count_three < vec.len() {
                        second_value.push(vec[count_three]);
                        count_three += 1;
                    }
                    let mut count_pow = 0;
                    let mut pow_sum = 1;
                    let fCV = first_value.parse::<i32>().unwrap();
                    let sCV = second_value.parse::<i32>().unwrap();
                    while count_pow < sCV {
                        pow_sum *= fCV;
                        count_pow += 1;
                    }
                    println!("The Result Is: {}", pow_sum);
                }
                count += 1;
            }
        } else {
            println!("Invalid Selection!");
        }
        println!("Do you want to continue program y/n ?");
        flag = read!();
    }
}
