use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {
    let mut number_of_guesses = String::new();
    println!("How man numbers would you like to guess?");
    io::stdin()
        .read_line(&mut number_of_guesses)
        .expect("TODO: panic message");

    let number_of_guesses: u8 = number_of_guesses.trim().parse().expect("Input error");
    let mut numbers_to_guess: Vec<u32> = Vec::new();
    for _ in 0..number_of_guesses {
        numbers_to_guess.push(rand::thread_rng().gen_range(1..=10));
    }
    println!("{numbers_to_guess:?}");
    println!("Generated vector with {} random numbers", numbers_to_guess.len());
    assert_eq!(numbers_to_guess.len(), number_of_guesses.into(), "Fatal Error!");

    let mut guess_count = 0;
    println!("Guess a number 1...10?");
    while guess_count < number_of_guesses {
        let mut guess_str = String::new();
        io::stdin()
            .read_line(&mut guess_str)
            .expect("TODO: panic message");

        let guess_num: u32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Parse error {e}");
                continue;
            }
        };

        //println!("Your guess: {guess_num}");
        // if expression
        match guess_num.cmp(&numbers_to_guess[guess_count as usize]) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                guess_count += 1;
                let remaining_guesses = number_of_guesses - guess_count;
                if remaining_guesses > 0 {
                    println!("Correct guess! {remaining_guesses} more to guess");
                }
            }
        };
    }
    println!("Thanks for playing. Here are the numbers you guessed:");
    for item in numbers_to_guess {
        println!("{item}")
    }
}
