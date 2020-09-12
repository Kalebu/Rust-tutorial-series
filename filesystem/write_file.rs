use std::fs::File;
use std::io::prelude::*;


fn main(){

    let quote = "Rust is the Language of the Future ";

    let mut file_o = File::create("guide.md")
        .expect("Can not create new file");

    file_o.write_all(quote.as_bytes())
        .expect("can not write info to file");

    println!("Wrote down to a file");
}