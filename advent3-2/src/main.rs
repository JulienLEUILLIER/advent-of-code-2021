use std::fs;
fn main() {
    let data = fs::read_to_string("data.txt").expect("Failed to read file");
    let lines: Vec<&str> = data.lines().collect();

    let mut oxygen = retain_appropriate_elements(lines.clone(), LifeSupports::Oxygen);
    let mut co2 = retain_appropriate_elements(lines.clone(), LifeSupports::Co2);

    println!(
        "{}",
        i32::from_str_radix(&oxygen.pop().unwrap().to_string(), 2).unwrap()
    );
}

enum LifeSupports {
    Oxygen,
    Co2,
}
struct BinaryAmount {
    zeros: u16,
    ones: u16,
}

fn retain_appropriate_elements(mut elements: Vec<&str>, lifesupport: LifeSupports) -> Vec<&str> {
    let line_length = elements.get(0).expect("Data provided is empty").len();

    for index in 0..line_length {
        let binary_amount = most_common_bit(&elements, index);

        let mut most_common_bit = '1';
        let mut least_common_bit = '0';

        if binary_amount.zeros > binary_amount.ones {
            most_common_bit = '0';
            least_common_bit = '1';
        } else if binary_amount.zeros <= binary_amount.ones {
            most_common_bit = '1';
            least_common_bit = '0';
        }

        elements.retain(|element| {
            let line_char = element
                .chars()
                .nth(index)
                .expect("No binary for this index");

            let element_retained = match lifesupport {
                LifeSupports::Oxygen => line_char == most_common_bit,
                LifeSupports::Co2 => line_char == least_common_bit,
            };

            element_retained
        });
    }

    elements
}

fn most_common_bit(elements: &Vec<&str>, index: usize) -> BinaryAmount {
    let mut binary_amount = BinaryAmount { zeros: 0, ones: 0 };

    for line in elements {
        let line_char = line.chars().nth(index).expect("No binary for this index");

        match line_char {
            '0' => binary_amount.zeros += 1,
            '1' => binary_amount.ones += 1,
            _ => (),
        };
    }

    binary_amount
}
