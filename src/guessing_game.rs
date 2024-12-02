use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let mut number = String::new();
    println!("Guess a number");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is, {}", secret_number);
    let number = match number.trim().parse::<i32>() {
        Ok(val) => val,
        Err(_) => {
            println!("Couldnt convert string to integer");
            return;
        }
    };
    match number.cmp(&secret_number) {
        Ordering::Less => println!("Lesser"),
        Ordering::Greater => println!("Greater"),
        Ordering::Equal => println!("You win"),
    }
}
