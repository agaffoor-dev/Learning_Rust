use std::io;

fn main() {
    println!("What number would you like to perform math operations on with the number 2?");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Error reading input");
    let number : u32 = number.trim().parse().expect("Error parsing");
    println!("You chose the number, {}.", number);
    println!("If you add 2 to it, you get {}", number + 2);
    println!("If you subtract 2 from it, you get {}", number - 2);
    println!("If you multiply it by 2, you get {}", number * 2);
    println!("If you divide it by 2, you get {}", number / 2);
}
