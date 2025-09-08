use std::{io};
use std::cmp::Ordering;
use rand::Rng;

fn main(){

    //title

    println!("Welcome to the guessing game!!");

    loop {
        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

        println!("Please enter your guess");
        // store the input, create a variable

        let mut guess = String::new();

        //store it
        io::stdin().read_line(&mut guess).expect("Failed to read the input");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => break,
        };//let guess

        println!("Your guess is: {guess} and the secret number is: {secret_number}");

        //check the entry and give op accordingly

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("Your guess is correct, guess number is equal to secret number");
                break;
            }//Ordering Equal
            
        }//match guess.cmp 
    }//loop
}