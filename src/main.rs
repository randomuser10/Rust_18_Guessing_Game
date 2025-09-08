use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Welcome to the guessing game");

    //generate the range
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("Please enter your guess");

    let mut guess = String::new();

    //store the input

    io::stdin().read_line(&mut guess).expect("Failed to read input");

    println!("Your guess is: {guess} and the secret number is: {secret_number}");

    //change the string to u32

    let guess: u32 = guess.trim().parse().expect("Please enter number only");

    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You win, guess is equal to secret number"),
        Ordering::Less => println!("Too small!!"),
        Ordering::Greater => println!("Too big!!"),
        
    }


}