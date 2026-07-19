# Allowing multiple guesses with looping

The guessing game now wraps the input-and-compare logic in a `loop`, letting the player keep
guessing until they get it right. Invalid input no longer crashes the program: parsing errors are
handled with a `match` expression that `continue`s the loop instead of calling `expect`. Guessing
correctly `break`s out of the loop.

Docs: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#allowing-multiple-guesses-with-looping
