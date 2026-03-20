// String and &str
fn main(){
    // String = vector of u8 (vec<u8>) valid UTF-8
    // &str = clice of u8 (&[u8]) valid UTF-8
    
    // when to use String vs &str
    // String -> mutate or data needs to be owned
    // &str -> read only
    
    // String 
    let msg: String = String::from("Hello rust");
    let len: usize = msg.len();
    println!("MSG:{}",msg);
    println!("LEN:{}",len);

    // str - string size
    //  &str
    //  - usually used str wit reference (borrowed)
    //  - immutable

    let msg: String = String::from("Hello");
    let s: &str = &msg[0..5];
    let len: usize = s.len();
    println!("Slice: {s}");
    println!("Len: {len}");
    let hello: &str = "Hello rust";
    // String literal
    // - stored inside binary
    // - slice pointing to a specific part of library
    // - immutable because hard-coded inside library
    let s: &str = r#"
            {"a": 1,
             "b": {"c":2},
             "c": 3
        "#;
    println!("Multiline string:{s}");

    let mut msg: String = "Hello rust".to_string();
    msg += "!";
    println!("{msg}");
    
    let name = "Suraj";
    let emoji = ":)";
    let msg = format!("Hello {name}{emoji}");
    println!("{msg}");
}
