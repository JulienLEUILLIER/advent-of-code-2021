use std::fs;

enum LifeSupports {
    Oxygen,
    Co2,
}
struct BinaryAmount {
    zeros: u16,
    ones: u16,
}
fn main() {
    let data = fs::read_to_string("data.txt").expect("Failed to read file");
    let lines: Vec<&str> = data.lines().collect();

    let mut oxygen = retain_appropriate_elements(lines.clone(), LifeSupports::Oxygen);
    let mut co2 = retain_appropriate_elements(lines.clone(), LifeSupports::Co2);

    println!(
        "{}",
        i32::from_str_radix(&oxygen.pop().unwrap().to_string(), 2).unwrap() *
        i32::from_str_radix(&co2.pop().unwrap().to_string(), 2).unwrap(),

    );
}


fn retain_appropriate_elements(mut elements: Vec<&str>, lifesupport: LifeSupports) -> Vec<&str> {
    let line_length = &elements[0].len();

    for index in 0..*line_length {
        let binary_amount = most_common_bit(&elements, index);

        if binary_amount.zeros == 0 || binary_amount.ones == 0 {
            break;
        }
        
        let most_common_bit:char;
        let least_common_bit:char;

        match binary_amount.zeros > binary_amount.ones {
            true => {
                most_common_bit = '0';
                least_common_bit = '1';
            }
            false => {
                most_common_bit = '1';
                least_common_bit = '0';
            }
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
