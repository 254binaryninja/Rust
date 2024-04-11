use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    loop {
        println!("Yooh woza guess a number my G (❁´◡`❁)");
        println!("Please input your guess.");

        let secret_number = rand::thread_rng().gen_range(1..=100);
        let secret_number = secret_number.to_string();

        let mut guess = String::new();

        loop {
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            // Trim the input and parse it as an integer
            match guess.trim().parse::<i32>() {
                Ok(_) => break, // Exit the loop when we get a number
                Err(_) => {
                    println!("This is not a valid number. Please enter a number.");
                    guess.clear(); // Clear the old input
                    continue; // Ask for input again
                },
            };
        }

        println!("You guessed: {}", guess.trim());
        match guess.trim().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
        println!("The secret number is: {}", secret_number);
    }
}