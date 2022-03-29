use std::fs;

fn main() {
    enum Direction {
        Forward,
        Up,
        Down,
        None
    }

    struct Controls {
        direction: Direction,
        amount: i32,
    }

    #[derive(Debug)]
    struct Submarine {
        horizontal: i32,
        depth: i32,
        aim: i32,
    }
    
    impl Submarine {
        fn set_aim(&mut self, amount: i32) -> () {
            self.aim += amount;
        }
        
        fn set_horizontal(&mut self, amount: i32) -> () {
            self.horizontal += amount;
            self.depth += self.aim * amount;
        }
    }
    
    let mut submarine = Submarine {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    let data = fs::read_to_string("data.txt").expect("Failed to read text file");

    let controls: Vec<Controls> = data
        .lines()
        .map(|line| {
            let line_values: Vec<&str> = line.split(' ').collect();
            let direction = match line_values[0] {
                "forward" => Direction::Forward,
                "up" => Direction::Up,
                "down" => Direction::Down,
                _ => Direction::None
            };

            let amount = line_values[1].parse::<i32>().expect("Failed to parse value");

            Controls { direction, amount }
        }).collect();

    for control in controls {
        match control.direction {
            Direction::Forward => submarine.set_horizontal(control.amount),
            Direction::Up => submarine.set_aim(-control.amount),
            Direction::Down => submarine.set_aim(control.amount),
            Direction::None => println!("Error : Unknown command"),
        }
    }
    
    println!("{:#?}", submarine);

    println!("{}", submarine.horizontal * submarine.depth);
}