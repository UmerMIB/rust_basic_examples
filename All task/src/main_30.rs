// task07
// Generate a Integer Vector until a certain condition is met
// Generate a random integer vector between 0 to 100 and keep on adding numbers to the vector until a number
// greater than 90 is generated. Once the vector is generated sort it.

use rand::Rng;
fn main(){
    let mut vec = Vec::new();
    
    loop{
    let mut a = rand::thread_rng().gen_range(0,101);
    if a<91{
        vec.push(a);
        }
        else{
            break;
        }
    }
    vec.sort();
    println!("{:?}",vec);
}
