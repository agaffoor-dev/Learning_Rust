use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("This is a guess the number game!");
    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("Guess a number in the range of 1 to 10");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading string");
        let guess:u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Type an integer please.");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
