fn main(){
    // Signed int
    // -(2**(n-1)) ~ 2**(n-1) - 1

    let i0: i8 = 0;
    // -(2**(8-1)) ~ 2**(8-1) - 1
    
    let i1: i16 = 1;
    // -(2**(16-1)) ~ 2**(16-1) - 1

    let i2: i32 = 11;
    // -(2**(32-1)) ~ 2**(32-1) - 1

    let i3: i64 = 111;
    // -(2**(64-1)) ~ 2**(64-1) - 1

    let i5: isize = 1111;
    // Depends on computer architecture
    
    // Unsigned integers
    // 0 ~ 2**n - 1
    let u1: u8 = 1;
    let u2: u16 = 2;
    let u3: u32 = 34;
    let u4: u64 = 567;
    let u5: u128 = 12345;
       
    // Floats

    let f0: f32 = 0.01;
    let f1: f64 = 0.02;

    // Boolean
    let b: bool = true;

    // Characters
    let c: char = 'c';
    let e: char = '🦀';
    
    // Type conversion
    let i: u32 = 1;
    let u: u32 = i as u32;
    let x: u32 = u + (i as u32);
    
    // MIN and MAX
    let min_i: i32 = i32::MIN;
    let max_i: i32 = i32::MAX;

    println!("i32 min:{}",min_i);
    println!("i32 max:{}",max_i);

    let min_char: char = char::MIN;
    let max_char: char = char::MAX;

    println!("Char min:{}",min_char);
    println!("Char max:{}",max_char);

    // Overflow
    let mut u: u32 = u32::MAX;
    u += 1;
    println!("Overflow u32{:?}",u);

    // Checked add - Some(x) | None
    let u = u32::checked_add(u32::MAX, 1);
    println!("Checked add: {}",u);
       
    // Wrapping add
    let u = u32::wrapping_add(u32::MAX, 1);
    println!("Wrapping add: {}",u);
}
       
       
       


