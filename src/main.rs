use std::{io, cmp::Ordering};
use rand::Rng;

fn main(){
    // title of the game
    println!("Welcome to the Guessing Game");

    //We want to continue till the user wants to quit or user wins
    loop {
        
        //generate a secret/random number
        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

        // take user input
        println!("Please enter your guess.");

        let mut guess = String::new();

        //store the input into the variable
        io::stdin().read_line(&mut guess).expect("Failed to read the message");

        // check if user wants to quit
        if guess.trim().eq_ignore_ascii_case("quit"){
            println!("Thank you for playing, see you next time. Bye for now!!");
            break;
        }


        //convert the string into u32 and give a message if the input is not a number
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }//Err(_)
        };//let guess: u32 = match guess.trim().parse()"


        //give the message of the guess and secret number

        println!("Your guess is: {guess} and the secret number is: {secret_number}");

        //match the secret/random number and give message accordingly

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!!"),
            Ordering::Less => println!("Too less!!"),
            Ordering::Equal => {
                println!("Congratulations!! You have won!!");
                break;
            }//Ordering::Equal
        }//match guess.cmp

    }//loop
}