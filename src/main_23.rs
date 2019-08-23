task23
Days Calculator
Write a Rust program to calculate number of days between two dates
Program Console Output:
Enter a date in (dd/mm/yy) format: 12/12/2018
Enter a date in (dd/mm/yy) format: 16/12/2018
There are 4 days in between 12/12/2018 and 16/12/18
Reference:
https://crates.io/crates/chrono

use chrono::{ NaiveDate};
use chrono::format::ParseError;

fn main() -> Result<(), ParseError> {
    println!("To get the difference between the dates please");
    let mut date1 = String::new();

    println!("Enter a date in (format 2015-09-05 YYY-MM-DD) ");
    io::stdin().read_line(&mut date1).expect("\nProblem reading data");

    let date_1 = NaiveDate::parse_from_str(&date1.trim(), "%Y-%m-%d")?;

    let mut date2 = String::new();

    println!("Enter a date in (format 2015-09-05 YYY-MM-DD) ");
    io::stdin().read_line(&mut date2).expect("\nProblem reading data");

    let date_2 = NaiveDate::parse_from_str(&date2.trim(), "%Y-%m-%d")?;

    let duration = date_2.signed_duration_since(date_1);

    println!("There are {:?} days in between {} and {}",duration.num_days(),date_1,date_2);

    Ok(())
}