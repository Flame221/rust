use std::io;
fn main() {
    println!("Hello, it's my first program on Rust :)");
    println!("Enter a name:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to readline");
    print!("Hello, {}", guess);
}