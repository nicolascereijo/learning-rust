//! # Main function
//!
//! These lines define a function named main. The main function is special: It is always the first
//! code that runs in every executable Rust program. Here, the first line declares a function named
//! main that has no parameters and returns nothing. If there were parameters, they would go inside
//! the parentheses (()).
//!
//! The function body is wrapped in {}. Rust requires curly brackets around all function bodies.
//! It’s good style to place the opening curly bracket on the same line as the function declaration,
//! adding one space in between.

fn main() {
    // First, println! calls a Rust macro. If it had called a function instead, it would be entered
    // as println (without the !). Rust macros are a way to write code that generates code to
    // extend Rust syntax, and we’ll discuss them in more detail in Chapter 20. For now, you just
    // need to know that using a ! means that you’re calling a macro instead of a normal function
    // and that macros don’t always follow the same rules as functions.
    //
    // Second, you see the "Hello, world!" string. We pass this string as an argument to println!,
    // and the string is printed to the screen.
    //
    // Third, we end the line with a semicolon (;), which indicates that this expression is over,
    // and the next one is ready to begin. Most lines of Rust code end with a semicolon.
    println!("Hello, world!");
}
