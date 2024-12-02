use std::io;

fn main() {
    let mut temperature_in_f = String::new();
    println!("Please input the temperature in farenheit:");
    io::stdin()
        .read_line(&mut temperature_in_f)
        .expect("Failed to read line");

    match temperature_in_f.trim().parse::<f32>() {
        Ok(num) => convert(num),
        Err(e) => println!("Failed to convert string to f32, {}", e),
    }
}

fn convert(temperatue_in_f: f32) {
    let temperature_in_c = (5.0 / 9.0) * (temperatue_in_f - 32.0);
    println!("The temperature in celcius is: {}", temperature_in_c)
}
