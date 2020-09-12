use std::fs::File;
use std::io::prelude::*;


fn main(){
        let mut empty_string = String::new();

        let mut file_o = File::open("guide.md")
            .expect("can not read file");

        file_o.read_to_string(&mut empty_string)
            .expect("failed to read to string");

        print!("{} \n",empty_string );
}