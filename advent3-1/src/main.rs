use std::fs;
fn main() {

    #[derive(Clone)]
    struct BinaryColumn {
        zeros: u16,
        ones: u16,
    }

    let data = fs::read_to_string("data.txt").expect("Failed to read file");

    let lines: Vec<&str> = data.lines().collect();

    let binary_line_len = lines[0].len();
    
    let mut binary_column_amount: Vec<BinaryColumn> = vec![BinaryColumn{zeros: 0, ones: 0}; binary_line_len];

    for line in lines {
        for index in 0..binary_line_len {

            let line_char = line.chars().nth(index).expect("No binary on this index");

            match line_char {
                '0' => binary_column_amount[index].zeros += 1,
                '1' => binary_column_amount[index].ones += 1,
                _ => println!("Non-binary number detected"),
            }
        }
    }

    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    for column in binary_column_amount {
        if column.zeros > column.ones {
            gamma.push('0');
            epsilon.push('1');
        } else if column.zeros < column.ones {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    let decimal_gamma = u32::from_str_radix(&gamma, 2).expect("Failed to convert to decimal");
    let decimal_epsilon = u32::from_str_radix(&epsilon, 2).expect("Failed to convert to decimal");
 
    println!("{}\n{}\n{}", decimal_gamma, decimal_epsilon, decimal_epsilon * decimal_gamma);
}