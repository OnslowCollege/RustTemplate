use std::io;

fn main() {
    println!("Enter your name:");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Hello, {}!", input.trim())
        }
        Err(error) => println!("Error getting name: {error}"),
    }
}