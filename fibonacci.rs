use std::io;

fn main(){
    println!("Enter the length of fibonacci terms to generate:");

    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read");

    let n: u32 = match input.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid positive integer.");
            return;
        }
    };
    generate_fibonacci(n);
}

fn generate_fibonacci(n:u32){
    let mut a: u128 = 0;
    let mut b: u128 = 1;

    println!("Fibonacci sequence ({} terms)",n);
    for i in 0..n{
        print!("{}",a);
        
        if i<n-1{
            print!(",");
        }
        let next = a+b;
        a=b;
        b=next;
    }
    println!()
}

