use std::fs;

struct BoardUnit {
    value: u8,
    marked: bool,
}

#[allow(unused_variables, dead_code)]
fn main() {
    let data = fs::read_to_string("data.txt").expect("Failed to read file");
    let mut drawn_numbers: Vec<u8> = Vec::new();
    let mut Boards: Vec<Vec<BoardUnit>> = Vec::new();

    for (index, line) in data.lines().enumerate() {
        if index == 0 {
            let mut numbers: Vec<u8> = line
                .split(',')
                .map(|num| num.parse().expect("Failed to parse number"))
                .collect();
            drawn_numbers.append(&mut numbers);
        }
    }
}
