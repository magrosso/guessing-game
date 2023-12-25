use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::process::exit;

fn main() {
    let number_to_guess = rand::thread_rng().gen_range(1..=10);
    println!("Random number: {number_to_guess}");
    println!("Guess a number 1...10?");
    let mut guess_count = 0;
    loop {
        let mut guess_str = String::new();
        io::stdin()
            .read_line(&mut guess_str)
            .expect("TODO: panic message");

        let guess_num: i32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Parse error {e}");
                continue;
            }
        };

        println!("Your guess: {guess_num}");
        guess_count += 1;
        // if expression
        match guess_num.cmp(&number_to_guess) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Correct guess after {guess_count} attempts!");
                break;
            }
        };
    }
}
