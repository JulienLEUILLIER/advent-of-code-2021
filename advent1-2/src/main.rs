use std::fs;

fn main() {
    let data = fs::read_to_string("../data.txt").expect("Failed to read file");

    let numbers: Vec<i32> = data.lines()
                                .map(|number| number.parse::<i32>().expect("Failed to parse number"))
                                .collect();

    let mut result: u32 = 0;
    let mut index: usize = 0;
    let mut previous_number: i32 = 2000000000; 

    while index < numbers.len() - 2 {

        let slice = &numbers[index..index + 3];

        let sum: i32 = slice.iter().sum();

        if previous_number < sum {
            result += 1;
        }
        index += 1;
        previous_number = sum;
    }

    println!("{}", result);
}