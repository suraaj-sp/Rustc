use std::io;

fn main(){
    let current_year = 2026;
    let mut name = String::new();
    let mut yob = String::new();

    println!("Enter your name:");
    io::stdin().read_line(&mut name).expect("Failed to read");

    println!("Enter year of birth:");
    io::stdin().read_line(&mut yob).expect("Failed to read");

    let year: i32 = yob.trim().parse().expect("Invalid year");
    let age = current_year - year;

    println!("Name: {}",name.trim());
    println!("Age: {}",age);

    if age>=60{
        println!("Senior");
    }
    else{
        println!("Not a senior");
    }
}
