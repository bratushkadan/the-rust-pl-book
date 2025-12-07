use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(0..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        // Shadowing is very useful when converting a variable type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // Err(_) means match ANY error
            Err(_) => {
                println!("Please, enter the number");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Number is too low!"),
            Ordering::Greater => println!("Number is too high!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            },
        }
    }
}
