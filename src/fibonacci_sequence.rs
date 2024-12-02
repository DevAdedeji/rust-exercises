use std::io;

fn main() {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    match number.trim().parse::<usize>() {
        Ok(num) => fibonnaci_sequence(num),
        Err(e) => println!("Failed to convert string to usize: {}", e),
    }
}

fn fibonnaci_sequence(num: usize) {
    let mut numbers = vec![0, 1];
    while numbers.len() < num {
        let numbers_length = numbers.len();
        let first_num = numbers[numbers_length - 1];
        let second_num = numbers[numbers_length - 2];
        let next_num = first_num + second_num;
        numbers.push(next_num);
    }
    println!("{:?}", numbers);
}
