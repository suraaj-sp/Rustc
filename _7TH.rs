#[derive(Debug)]

// Struct

struct Point {
    x: f32,
    y: f32,
}

struct Point3d(f32,f32,f32);

struct Empty;

#[derive(Debug)]
struct Circle{
    centre: Point,
    radius: u32,
}

fn main(){
    // Create
    let p = Point { x:1.0, y:2.0};
    println!("x={},y={}",p.x,p.y);
    let p = Point3d (1.0,2.0,3.0);
    println!("Point 3d, {}, {}, {}",p.0,p.1,p.2);
    
    let empty = Empty;

    let circle = Circle{
        centre: Point {x: 1.0, y:2.0},
        radius: 1,
    };
    // Debug
    // Read
    println!("{:?}, {:?}",circle.centre, circle.radius);
    
    // Shortcut
    let x = 1.0;
    let y = 1.0;
    let p = Point {x:x,y:y};
    println!("{},{}",p.x,p.y);

    // Copy Fields
    let p0 = Point {x:5.0 , y:7.0};
    println!("{},{}",p0.x,p0.y);

    // Update
    let mut p = Point {x:3.0,y:9.0};
    p.x += 1.0;
    p.y += 8.0;
    println!("{},{}",p.x,p.y);
}
