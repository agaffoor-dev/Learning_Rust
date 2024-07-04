use std::io;
use rand::Rng;

fn main() {
    println!("This is a guess the number game!");
    let num = rand::thread_rng().gen_range(1..11);
    println!("Guess a number in the range of 1 to 10");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading string");
    let guess:u16 = guess.trim().parse().expect("Invalid");
    println!("The winning number is {}" ,num);
    println!("You guessed {}", guess);
}
