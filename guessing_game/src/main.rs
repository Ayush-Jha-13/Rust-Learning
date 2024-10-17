use std::io;
use rand::Rng;

fn main() {
    println!("Guess the Number!");

    // Generate a secret number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the guess from a string to an integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        if guess == secret_number {
            println!("You guessed correctly!");
            break;
        } else if guess < secret_number {
            println!("Too small!");
        } else {
            println!("Too big!");
        }
    }
}
