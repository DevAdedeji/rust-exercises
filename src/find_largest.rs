fn main() {
    //
    let numbers = [1, 2, 3445, 34555, 3555];
    match find_largest(&numbers) {
        Some(largest) => println!("The largest number is: {}", largest),
        None => println!("The array is empty"),
    }
}

fn find_largest(numbers: &[i32]) -> Option<i32> {
    if numbers.len() == 0 {
        println!("Empty array");
        return None;
    }
    let mut largest = numbers[0];
    for &num in numbers {
        if num > largest {
            largest = num;
        }
    }
    Some(largest)
}
