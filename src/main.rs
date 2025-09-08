use std::io;

fn main(){
    println!("Welcome to the guessing game");

    println!("Please enter your guess");

    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).expect("Failed to read the number");

    println!("Your guess is: {input_str}");
}