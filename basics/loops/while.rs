
fn main(){

    let mut x = 0;

    while x<10 {
        println!("hello Rust , {}", x);
        if x==9 {
            break;
        }
        x+=1;
    }

}