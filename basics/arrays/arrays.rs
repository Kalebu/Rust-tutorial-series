// Simple involving arrays manipulation

fn main(){

    let students = ["Learning ", "Rust ", "is ", "really ", "interesting"];

    //____________printing all elements_______
    println!("Array content : {:?}", students);

    
    //___________getting array length____________
    println!("Array Length  : {}", students.len());

    
    //___________getting individual elements______

    println!("First item : {} Last  item : {}", students[0], students[students.len() - 1])






}