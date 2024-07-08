use std::io;

fn main() {
    println!("This is the number guessing game!");
    println!("Enter a number in the range 1 to 10 below:");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading input");
    let guess : u16 = guess.trim().parse().expect("Error parsing");
    println!("You guessed {}", guess);
}
