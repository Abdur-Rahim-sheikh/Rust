use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("Guess the number");
    let secret_number = rand::rng().random_range(1..101);
    println!("The secret number is: {}", secret_number);
    loop{
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_) => continue,
        };
        println!("You guessd: {}", guess);
        let msg = match guess.cmp(&secret_number) {
            Ordering::Less => "Too Small".red(),
            Ordering::Equal => "You win!".green(),
            Ordering::Greater => "Too big".yellow(),
        };

        println!("{}", msg);
        if msg=="You win!".green(){
            break;
        }
    }
    
}
