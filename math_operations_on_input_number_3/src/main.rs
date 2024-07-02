use std::io;

fn main() {
    println!("Enter a number for me to add 3 to: ");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Error");
    let number:i32 = number.trim().parse().expect("Error");
    println!("After adding 3, the sum is {}.", number + 3);
    println!("After multiplying by 3, the product is {}.", number * 3);
    println!("After dividing by 3, the quotient is {}.", number / 3);
    println!("After subtracting 3, the difference is {}.", number - 3);
}
