use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number");
    let secret_number = rand::rng().random_range(1..101);
    println!("The secret number is: {}", secret_number);
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Faild to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessd: {}", guess);
    let msg = match guess.cmp(&secret_number) {
        Ordering::Less => "Too Small",
        Ordering::Equal => "You win!",
        Ordering::Greater => "Too big",
    };

    println!("{}", msg);
}
