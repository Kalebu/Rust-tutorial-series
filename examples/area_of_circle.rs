use std::io;

fn main()
{
    const PI:f32 = 3.14159;
    let mut radius_str = String::new();
    
    println!("Enter Radius of the Circle  ");
    io::stdin().read_line(&mut radius_str).expect("expected string");
    let radius:f32 = radius_str.trim().parse().expect("expecting float");

    let area  = PI*radius*radius;
    println!("The area of with radius of {} the circle is {} ", radius, area);

}