use std::io;

// Nico: Here's a behavior that's confusing at first. Even though 'rand' is already imported, we end
// up using it again in the call to the function that generates the random range. This happens
// because that function is defined inside a 'trait'.
// This gets explained later, but a 'trait' is something like a list of things a 'type' promises to
// have, such as methods. It needs to be imported wherever it's used to have visibility, because by
// design Rust forces this to avoid naming confusion if several traits define similar names. It's
// not the same thing, but roughly speaking it's like having an interface that groups a set of
// functions together, and any 'type' that wants to use those functions has to expose that
// interface, so the interface itself must be imported even though the 'type' already specifies its
// use.
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // First, we call the 'rand::thread_rng' function that gives us the particular random number
    // generator we’re going to use: one that is local to the current thread of execution and is
    // seeded by the operating system. Then, we call the 'gen_range' method on the random number
    // generator. This method is defined by the 'Rng' 'trait' that we brought into scope with the
    // use 'rand::Rng;' statement.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
