use std::{io, cmp::Ordering};
use rand::Rng;

fn main(){
// title
    println!("Welcome to the guessing game!!!");
//game should continue unless user want to quit or user wins
    loop {

        //generate a random/secret number
        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

        println!("Please enter your guess.");

        //create a variable to store the input

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read the input");

        //check if the user wants to quit the game
        if guess.trim().eq_ignore_ascii_case("quit"){
            println!("Thank you for playing the game, Bye for now!!");
            break;
        }//if guess.trim().eq_ignore_ascii.case("quit")

        //change the input from string to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("Please enter a numerical value!");
                        continue;
            }
        };

        println!("Your guess is: {guess} and the secret number is: {secret_number}");

        //match the guess and secret number and give the output accordingly.

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!!"),
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("Congratulations!!, Your guess is correct, you win!!");
                break;
            }//Ordering::Equal
            
        }//match guess.cmp
        
    }//loop

}