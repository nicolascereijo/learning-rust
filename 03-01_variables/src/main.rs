/* fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
} */

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

// Shadowing is different from marking a variable as 'mut' because we’ll get a compile-time error if
// we accidentally try to reassign to this variable without using the 'let' keyword. By using 'let',
// we can perform a few transformations on a value but have the variable be immutable after those
// transformations have completed.

// The other difference between 'mut' and shadowing is that because we’re effectively creating a new
// variable when we use the 'let' keyword again, we can change the type of the value but reuse the
// same name. For example, say our program asks a user to show how many spaces they want between some
// text by inputting space characters, and then we want to store that input as a number:
//
//     let spaces = "   ";
//     let spaces = spaces.len();
//
// The first spaces variable is a string type, and the second spaces variable is a number type.
// Shadowing thus spares us from having to come up with different names, such as 'spaces_str' and
// 'spaces_num'. Instead, we can reuse the simpler spaces name. However, if we try to use 'mut' for
// this, we’ll get a compile-time error.
