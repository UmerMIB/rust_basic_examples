// task11
// Adding Values in a Loop
// Write a program that gets several integers from the user. Sum up all the integers they give you. Stop looping when
// they enter a 0. Display the total at the end.
// Program Console Sample 1:
// I will add up the numbers you give me.
// Number: 6
// The total so far is 6
// Number: 9
// The total so far is 15
// Number: -3
// The total so far is 12
// Number: 2
// The total so far is 14
// Number: 0
// The total is 14.
// Program Console Sample 2:
// I will add up the numbers you give me.
// Number: 1
// The total so far is 1
// Number: 2
// The total so far is 3
// Number: 3
// The total so far is 6
// Number: 4
// The total so far is 10
// Number: 5
// The total so far is 15
// Number: 0
// The total is 15

fn main(){
    let mut sum:f32 = 0.0;
    loop{
        println!("I will add up the numbers you give me.");
        
        println!("Number 1 ");
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("\nProblem reading text");

        let inp_i: f32 = match inp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nWrong input for radius");
                continue;
            }
        };
            sum += inp_i;
            if inp_i !=0.0{
                println!("The total so far is {}",sum);
            }
            else if inp_i == 0.0 {
                println!("The total so far is {}",sum);
                break;
            }
    }
}