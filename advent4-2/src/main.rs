use std::fs;

struct BoardUnit {
    value: u32,
    marked: bool,
}

impl BoardUnit {
    fn new(value: u32) -> BoardUnit {
        BoardUnit {
            value: value,
            marked: false,
        }
    }
}

struct Board {
    board: Vec<Vec<BoardUnit>>,
    winner: bool,
}

impl Board {
    fn new() -> Board {
        Board {
            board: vec![],
            winner: false,
        }
    }

    fn mark_number_as_true(&mut self, number: u32) {
        let board_line_length = self.board.len();

        for i in 0..board_line_length {
            for j in 0..board_line_length {
                let unit = &mut self.board[i][j];
                if unit.value == number {
                    unit.marked = true;
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        let board_line_length = self.board.len();
        let mut row = 0;
        let mut column = 0;
        let mut winner = false;

        for i in 0..board_line_length {
            for j in 0..board_line_length {
                if self.board[i][j].marked {
                    row += 1;
                }
            }
            if row == board_line_length {
                winner = true;
            }
            row = 0;
        }

        for i in 0..board_line_length {
            for j in 0..board_line_length {
                if self.board[j][i].marked {
                    column += 1;
                }
            }
            if column == board_line_length {
                winner = true;
            }
            column = 0;
        }

        winner
    }

    fn sum_of_unmarked_numbers_in_winner_board(&self) -> u32 {
        let board_line_length = self.board.len();
        let mut sum: u32 = 0;

        for i in 0..board_line_length {
            for j in 0..board_line_length {
                let unit = &self.board[i][j];
                if !unit.marked {
                    sum += unit.value;
                }
            }
        }

        sum
    }
}
fn main() {
    let data = fs::read_to_string("data.txt").expect("Failed to read file");
    let mut drawn_numbers: Vec<u32> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();
    let mut current_board = 0;
    let mut last_winner_board_index: i32 = -1;
    let mut winning_drawn_number = 0;
    let mut sum_of_unmarked_numbers_winner_board = 0;

    for (index, line) in data.lines().enumerate() {
        if index == 0 {
            let mut numbers: Vec<u32> = line
                .split(',')
                .map(|num| num.parse().expect("Failed to parse number"))
                .collect();
            drawn_numbers.append(&mut numbers);
        }

        if line.trim().is_empty() {
            boards.push(Board::new());
            if index > 2 {
                current_board += 1;
            }
            continue;
        }

        if index > 1 {
            let row: Vec<BoardUnit> = line
                .split_whitespace()
                .map(|num| {
                    BoardUnit::new(
                        num.parse()
                            .expect("Failed to parse number to create new BoardUnit"),
                    )
                })
                .collect();
            boards[current_board].board.push(row);
        }
    }

    for drawn_number in drawn_numbers {
        for (index, board) in boards.iter_mut().enumerate() {
            if !board.winner {
                board.mark_number_as_true(drawn_number);

                if board.is_winner() {
                    last_winner_board_index = index as i32;
                    winning_drawn_number = drawn_number;
                    board.winner = true;
                    sum_of_unmarked_numbers_winner_board =
                        board.sum_of_unmarked_numbers_in_winner_board();
                }
            }
        }
    }

    println!(
        "{}, {}, {}",
        last_winner_board_index,
        winning_drawn_number,
        sum_of_unmarked_numbers_winner_board * winning_drawn_number
    );
}
