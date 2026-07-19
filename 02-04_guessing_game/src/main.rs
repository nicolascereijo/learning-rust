use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    // --snip--
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Rust allows us to shadow the previous value of guess with a new one.
    // We bind this new variable to the expression 'guess.trim().parse()'. The 'guess' in the
    // expression refers to the original 'guess' variable that contained the input as a string. The
    // 'trim' method on a 'String' instance will eliminate any whitespace at the beginning and end,
    // which we must do before we can convert the string to a 'u32', which can only contain
    // numerical data.
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    // The 'cmp' method compares two values and can be called on anything that can be compared. It
    // takes a reference to whatever you want to compare with: Here, it’s comparing 'guess' to
    // 'secret_number'. Then, it returns a variant of the 'Ordering' 'enum' we brought into scope
    // with the 'use' statement. We use a 'match' expression to decide what to do next based on
    // which variant of 'Ordering' was returned from the call to 'cmp' with the values in 'guess'
    // and 'secret_number'.
    // A 'match' expression is made up of arms. An 'arm' consists of a pattern to match against, and
    // the code that should be run if the value given to match fits that arm’s pattern. Rust takes
    // the value given to match and looks through each arm’s pattern in turn. Patterns and the match
    // construct are powerful Rust features: They let you express a variety of situations your code
    // might encounter, and they make sure you handle them all.

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
