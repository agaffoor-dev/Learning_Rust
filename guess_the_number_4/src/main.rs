use std::io;

fn main() {
    println!("This is a guess the number game!");
    println!("Guess a number in the range of 1 to 10");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading string");
    let guess:u16 = guess.trim().parse().expect("Invalid");
    println!("You guessed {}", guess);
}
