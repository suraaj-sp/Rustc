use std::io;

struct Student{
    name: String,
    usn: String,
    marks: [f32;3],
}

fn main(){
    let _input = String::new();
    println!("Enter student details:");
    println!("Name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed to read");

    println!("Enter usn:");
    let mut usn = String::new();
    io::stdin().read_line(&mut usn).expect("Failed to read");

    let mut marks = [0.0;3];
    for i in 0..3{
        println!("Enter marks for subject {}:",i+1);
        let mut m_str = String::new();
        io::stdin().read_line(&mut m_str).expect("Failed to read");
        marks[i] = m_str.trim().parse().expect("please enter a valid number");
    }

    let student = Student{name:name.trim().to_string(),usn:usn.trim().to_string(),marks};
    let total: f32 = student.marks.iter().sum();
    let percentage = (total/300.0)*100.0;
    println!("student report card");
    println!("name {}",student.name);
    println!("usn {}",student.usn);
    println!("Total {}",total);
    println!("percentage {:.2}%",percentage);
}
