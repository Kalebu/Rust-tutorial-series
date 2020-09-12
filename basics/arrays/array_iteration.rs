

fn main(){

    // Arrays of Fruits 

    let fruits = ["Mango", "Apple", "Orange"];


    // Array Iteration and printing each element in iteration

    for fruit in fruits.iter(){
        println!("{}", fruit);
    }

    
    //Using loops 

    let mut index = 0;

    loop {
            println!("{}", fruits[index]);
            if index==2{
                break;
            }   
            index+=1;
    }


}