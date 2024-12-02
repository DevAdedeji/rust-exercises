use std::io;
fn main() {
    let mut num1 = String::new();
    println!("Enter the first number");
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");

    let num1: f64 = match num1.trim().parse::<f64>() {
        Ok(val) => val,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    let mut num2 = String::new();
    println!("Enter the second number");
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");

    let num2: f64 = match num2.trim().parse::<f64>() {
        Ok(val) => val,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    let mut operator = String::new();
    println!("Enter operation, (+, -, *, /)");
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");

    let result = match operator.trim() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Cannot divide by zero");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    println!("The result is: {}", result);
}
