// The 'io' library comes from the standard library, known as 'std'.
// By default, Rust has a set of items defined in the standard library that it brings into the scope
// of every program. This set is called the prelude.
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // We use the 'let' statement to create the variable. In Rust, variables are immutable by default,
    // meaning once we give the variable a value, the value won’t change.
    // To make a variable mutable, we add 'mut' before the variable name.
    let mut guess = String::new();

    // On the right of the equal sign is the value that guess is bound to, which is the result of
    // calling 'String::new', a function that returns a new instance of a 'String'. 'String' is a
    // string type provided by the standard library that is a growable, UTF-8 encoded bit of text.

    // 'growable' means 'String' does not have a fixed size, it can grow or shrink at runtime as
    // text is added or removed.
    // This is different from the other string type in Rust, the string slice '&str', which has a
    // fixed size known at compile time and cannot grow.
    // 'String' stores its data on the heap while '&str' is usually a reference to data stored
    // elsewhere, such as a string literal baked into the binary.
    // 'new' is an associated function of the 'String' type, called with '::', it creates a new
    // empty 'String'.

    // Rust programs use two main areas of memory, the 'stack' and the 'heap'. These two areas of
    // memory are not unique to Rust, most programming languages work with some version of the
    // 'stack' and the 'heap'.
    // The 'stack' is fast and stores values with a fixed size known at compile time, such as 'i32',
    // 'bool' or the '&str' reference itself.
    // The 'heap' is for values whose size can change at runtime, such as 'String'. Reserving space
    // there takes a bit longer since Rust has to find a free spot and keep a pointer back to it.
    //
    // A 'String' is actually made of two parts.
    //
    // Stack                     Heap
    // ┌─────────────┐          ┌──────────────┐
    // │ pointer    ─┼─────────>│ H e l l o    │
    // │ len: 4      │          └──────────────┘
    // │ capacity: 8 │
    // └─────────────┘
    //
    // The 'stack' part holds a pointer to the data, its length and its capacity, all fixed size.
    // The 'heap' part holds the actual text, which is what allows it to grow.
    // Rust frees the 'heap' memory automatically once the owning variable goes out of scope, no
    // garbage collector needed.

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // If we hadn’t imported the 'io' module with use 'std::io;' at the beginning of the program, we
    // could still use the function by writing this function call as 'std::io::stdin'. The 'stdin'
    // function returns an instance of 'std::io::Stdin', which is a type that represents a handle to
    // the standard input for your terminal.

    // The '&' indicates that this argument is a reference, which gives you a way to let multiple
    // parts of your code access one piece of data without needing to copy that data into memory
    // multiple times. Like variables, references are immutable by default. Hence, you need to write
    // '&mut' guess rather than '&guess' to make it mutable.

    // Nico: This is a big difference compared to Python or C.
    // In Python, it's common to run into problems with references and to need a deep copy to avoid
    // altering the original data, a typical bug is modifying a list or dict inside a function and
    // finding out it changed the original because it was passed by reference. Pointers in C can
    // also cause issues, but the other way around, function parameters never reference the original
    // if passed normally, so a typical bug there is making changes to a copy thinking it's the
    // original, when you actually needed a pointer to modify it.

    // As mentioned earlier, 'read_line' puts whatever the user enters into the string we pass to
    // it, but it also returns a 'Result' value. 'Result' is an enumeration, often called an enum,
    // which is a type that can be in one of multiple possible states. We call each possible state a
    // variant.

    println!("You guessed: {guess}");
}
