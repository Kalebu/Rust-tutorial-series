use std::io;
use rand::Rng;


fn main() {
        let mut r_gen = rand::thread_rng(); 
        println!("__________Guessing game___________________");

        loop {
             let random_n:u32 = r_gen.gen_range(1, 100);

             loop {
                    let mut  guess_str = String::new();
                    println!("Guess a number ");
                    io::stdin().read_line(&mut guess_str).expect("string value");
                    let guess:u32 = guess_str.trim().parse().expect("expect integer value");
                    if guess == random_n{
                        println!("congratulation you made it \n Play again \n\n");
                        break;
                    }
                    else if guess>random_n{
                        println!("The number you guessed it too high");
                    }

                    else{
                        println!("The number is too low ");
                    }
                
             }
        }
}
