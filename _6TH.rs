#![allow(unused)]
#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
    Hex(String),
    Hsl { h: u8, s:u8, l:u8 },
}
fn main(){
    // Enum
    let color: Color= Color::Red;
    let color = Color::Green;
    let color = Color::Rgba(0, 0, 255, 0.1);
    let color = Color::Hex("#ffffff".to_string());
    let color = Color::Hsl {h:1,s:2,l:3};

    // Attributes - Debug and partialEQ
    // Debug
    println!("{:?}",color);
    // PartialEq
    println!("{}",Color::Red == Color::Green);
    println!("{}",Color::Red == Color::Red);

    // Option = Some(11) | None
    let x: Option<i32> = None;
    let x: Option<i32> = Some(-11);
    println!("option: {:?}", x);
    // Result = Ok(10) | Err("Div by zero"
    let res: Result<u32, String> = Err("Div by 0".to_string());
    println!("Result: {:?}", res);

}

