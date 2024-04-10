use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    loop{
    println!("Yooh woza guess a number my G (❁´◡`❁)");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let secret_number = secret_number.to_string();

     let mut guess = String::new();

     io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read line");

        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
        println!("The secret number is: {secret_number}")
    }
}