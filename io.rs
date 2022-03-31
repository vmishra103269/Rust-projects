// Rust input output example
use std::io;

fn main() {
    println!("Enter a name:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to readline");
    println!("You entered {}",guess);
}
