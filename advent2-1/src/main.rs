use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt").expect("Failed to read the file");

    #[derive(Debug)]
    struct Position {
        horizontal: i32,
        vertical: i32,
    }

    impl Position {
        fn set_depth(&mut self, amount: i32) -> () {
            self.vertical += amount;
        }

        fn set_horizontal(&mut self, amount: i32) -> () {
            self.horizontal += amount;
        }
    }

    let mut submarine = Position {
        horizontal: 0,
        vertical: 0,
    };

    for line in data.lines() {
        let words: Vec<&str> = line.split(' ').collect();

        let amount = words[1].parse::<i32>().expect("Failed to parse number");

        match words[0] {
            "forward" => submarine.set_horizontal(amount),
            "up" => submarine.set_depth(-amount),
            "down" => submarine.set_depth(amount),
            _ => ()
        }
    }

    println!("{:#?}", submarine);

    println!("{}", submarine.horizontal * submarine.vertical);
}
