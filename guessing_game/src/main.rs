use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Welcome to the Guessing game!");
    let secret: u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Enter your Guess Below:");
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read Line");
        
        if guess == "quit\n" {
            println!("Thanks for playing!");
            break;
        }
        let guess : u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("Enter a number");
                continue;
            }
        };
       
        match guess.cmp(&secret){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}
