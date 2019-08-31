// task60
// Generate a Float Vector and Count Elements
// Generate a Vector of 100 random floats between 0 and 900. Find out how many of the random numbers are
// between 0 to 300, 300 to 600, and above 600. 

use rand::Rng;
fn main(){
    let mut vec = Vec::new();
    for a in 0..100{
        vec.push(rand::thread_rng().gen_range(0.0,900.0));
    }
    quickersort::sort_floats(&mut vec[..]);
    
    let mut num_a = 0;
    let mut num_b = 0;
    let mut num_c = 0;
    for a in 0.. vec.len(){
        if vec[a]>0.0 && vec[a]<300.0{
            num_a += 1;
        }
        else if vec[a]>300.0 && vec[a]<600.0{
            num_b += 1;
        }
        else{
            num_c+= 1;
        }
    }
    println!("Numbers between 0-300 are{}\nNumbers between 300-600 are{} \n Numbers above 600 are{}\n",num_a,num_b,num_c);
}
