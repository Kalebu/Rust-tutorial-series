use std::io;

fn main(){

    let mut  First_no_str = String::new();
    let mut Second_no_str = String::new();
    let mut operation_str = String::new();

    println!("Enter the First number ");
    io::stdin().read_line(&mut First_no_str).expect("string value");
    println!("Enter Operation ");
    io::stdin().read_line(&mut operation_str).expect("operator expected");
    println!("Enter the second number");
    io::stdin().read_line(&mut Second_no_str).expect("string value");
    
    let first_number:i32 = First_no_str.trim().parse().expect("This isn't a number fam");
    let second_number:i32 = Second_no_str.trim().parse().expect("This isn't a number fam");


    if operation_str.trim() == "+"
    {
        let result = first_number + second_number;
        println!("The answer is {}",result);
    }
}