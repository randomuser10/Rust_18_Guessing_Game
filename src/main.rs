use std::io;

use rand::Rng;


fn main(){
    println!("Welcome to the guessing game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please enter your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read the number");

    println!("Your guess is: {guess} and the secret number is: {secret_number}.");
}