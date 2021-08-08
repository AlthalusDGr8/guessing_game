use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Starting Game!");
    println!("Guess the number!");

    let small_number: u32 = 1;
    let big_number: u32 = 20;

    println!(
        "I am thinking of a number between {} and {}.",
        small_number, big_number
    );

    let secret_number = rand::thread_rng().gen_range(small_number..big_number);
    let mut number_of_guesses: i32 = 1;

    //  println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("You Guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("too small!");
                number_of_guesses += 1;
            }

            Ordering::Greater => {
                println!("too big!");
                number_of_guesses += 1;
            }

            Ordering::Equal => {
                println!(
                    "You Win!. Total number of guesses needed was {}",
                    number_of_guesses
                );
                break;
            }
        }
    }
}
