use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // We switch from an 'expect' call to a 'match' expression to move from crashing on an error
        // to handling the error. Remember that 'parse' returns a 'Result' type and 'Result' is an
        // 'enum' that has the variants 'Ok' and 'Err'. We’re using a 'match' expression here, as we
        // did with the 'Ordering' result of the 'cmp' method.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
