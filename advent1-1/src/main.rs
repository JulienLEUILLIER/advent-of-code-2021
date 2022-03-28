use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt").expect("Failed to read file");
    let mut result = 0;
    let mut previous_number: i32 = 2000000000; 

    for line in data.lines() {

        let number = line.parse::<i32>().expect("Failed to parse number");

        if previous_number < number {
            result += 1;
        }

        previous_number = number;
    }

    println!("{}", result);
}
