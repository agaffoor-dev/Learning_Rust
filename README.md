# Learning_Rust
### Reading official documentation and "The Rust Programming Book"
- - -
##### A Hello World Program
- I was able to successfully use cargo and get the cli to print "Hello, world!" using the default files created through `cargo new`.
- - -
##### Greeting
- Here is where I have been introduced to the feature of mutable and immutable variables in rust as well as modules called crates. I was able to reproduce the code for greeting a person after reading through the documentation several times.
- - -
##### Performing Basic Operations on Number Input
- I found that working with numbers is a little more tricky than inputting strings as I look through the guess the number example in the docs. Interestingly, if it does not interact with another number, for example `number + 4`, it doesn't throw an error since it is treated as a string. To treat the input as a number, I had to convert an input to a i32 type number, trim followed by parse followed by expect has to be done. I know trim removes surrounding whitespace and parse interprets the string as a number, but I still have to get an understanding of what 'expect' does.
- - -
##### Guess the Number Game
- I looked through the guess the number game example a few days back. Now I will try to replicate it based on my understanding of rust concepts I have learned so far. So far, I am able to use the rand crate to generate a random number between 1 to 10. Now, I will have to learn pattern matching in rust before proceeding to create a loop.
