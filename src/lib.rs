use colored::Colorize;
use std::fmt::Display;

const GRID_SIZE: usize = 9;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Block {
    Empty,
    Val(u8),
    Fix(u8),
}

#[derive(Debug)]
pub struct Board([[Block; GRID_SIZE]; GRID_SIZE]);

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if self.0[i][j] != other.0[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

impl Board {
    pub fn new() -> Self {
        Board([[Block::Empty; GRID_SIZE]; GRID_SIZE])
    }

    pub fn from(input: [[u8; GRID_SIZE]; GRID_SIZE]) -> Self {
        let mut board = Board::new();
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                match input[i][j] {
                    0 => board.0[i][j] = Block::Empty,
                    n if n >= 1 && n <= 9 => board.0[i][j] = Block::Fix(n),
                    _ => board.0[i][j] = Block::Empty,
                }
            }
        }
        board
    }

    pub fn from_str(input: [&str; GRID_SIZE]) -> Self {
        let mut board = Board::new();
        for i in 0..GRID_SIZE {
            for (index, val) in input[i].chars().enumerate() {
                board.0[i][index] = match val.to_digit(10).expect("only numbers are accepted") {
                    n if n > 0 && n <= 9 => Block::Fix(n as u8),
                    _ => Block::Empty,
                }
            }
        }
        board
    }

    pub fn is_number_in_row(&self, number: u8, row: u8) -> bool {
        for i in 0..GRID_SIZE {
            match self.0[row as usize][i] {
                Block::Fix(n) | Block::Val(n) => {
                    if n == number {
                        return true;
                    }
                }
                _ => continue,
            }
        }
        false
    }

    pub fn is_number_in_column(&self, number: u8, column: u8) -> bool {
        for i in 0..GRID_SIZE {
            match self.0[i][column as usize] {
                Block::Fix(n) | Block::Val(n) => {
                    if n == number {
                        return true;
                    }
                }
                _ => continue,
            }
        }
        false
    }

    fn is_number_in_box(&self, number: u8, row: u8, column: u8) -> bool {
        let local_box_row = row - row % 3;
        let local_box_column = column - column % 3;

        for i in local_box_row..(local_box_row + 3) {
            for j in local_box_column..(local_box_column + 3) {
                match self.0[i as usize][j as usize] {
                    Block::Fix(n) | Block::Val(n) => return n == number,
                    _ => continue,
                }
            }
        }
        false
    }

    fn is_valid_placement(&self, number: u8, row: u8, column: u8) -> bool {
        !self.is_number_in_row(number, row)
            && !self.is_number_in_column(number, column)
            && !self.is_number_in_box(number, row, column)
    }

    pub fn solve(&mut self) -> bool {
        for row in 0..GRID_SIZE {
            for column in 0..GRID_SIZE {
                match self.0[row][column] {
                    Block::Empty => {
                        for number_to_try in 1..=GRID_SIZE {
                            if self.is_valid_placement(number_to_try as u8, row as u8, column as u8)
                            {
                                self.0[row][column] = Block::Val(number_to_try as u8);

                                if self.solve() {
                                    return true;
                                } else {
                                    self.0[row][column] = Block::Empty;
                                }
                            }
                        }
                        return false;
                    }
                    _ => continue,
                }
            }
        }
        true
    }
}
impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Block::Empty => ".".to_string(),
            Block::Val(v) => format!("{}", v).bright_blue().to_string(),
            Block::Fix(v) => format!("{}", v).red().to_string(),
        };
        write!(f, "{val}")
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = String::from("═").repeat(11);
        let _ = writeln!(f, "╔{}╦{}╦{}╗", line, line, line);
        for i in 0..GRID_SIZE {
            if i % 3 == 0 && i != 0 {
                let _ = writeln!(f, "╠{}╬{}╬{}╣", line, line, line);
            }
            for j in 0..GRID_SIZE {
                let _ = if j % 3 == 0 {
                    write!(f, "║")
                } else {
                    write!(f, "|")
                };
                let _ = write!(f, " {} ", self.0[i][j]);
            }
            let _ = writeln!(f, "║");
        }
        let _ = writeln!(f, "╚{}╩{}╩{}╝", line, line, line);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Block, Board, GRID_SIZE};
    const SAMPLE: Board = Board([
        [
            Block::Fix(7),
            Block::Empty,
            Block::Fix(2),
            Block::Empty,
            Block::Fix(5),
            Block::Empty,
            Block::Fix(6),
            Block::Empty,
            Block::Empty,
        ],
        [
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Fix(3),
            Block::Empty,
            Block::Empty,
            Block::Empty,
        ],
        [
            Block::Fix(1),
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Fix(9),
            Block::Fix(5),
            Block::Empty,
            Block::Empty,
        ],
        [
            Block::Fix(8),
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Fix(9),
            Block::Empty,
        ],
        [
            Block::Empty,
            Block::Fix(4),
            Block::Fix(3),
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Fix(7),
            Block::Fix(5),
            Block::Empty,
        ],
        [
            Block::Empty,
            Block::Fix(9),
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Fix(8),
        ],
        [
            Block::Empty,
            Block::Empty,
            Block::Fix(9),
            Block::Fix(7),
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Fix(5),
        ],
        [
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Fix(2),
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Empty,
            Block::Empty,
        ],
        [
            Block::Empty,
            Block::Empty,
            Block::Fix(7),
            Block::Empty,
            Block::Fix(4),
            Block::Empty,
            Block::Fix(2),
            Block::Empty,
            Block::Fix(3),
        ],
    ]);

    #[test]
    fn fill_board() {
        let board = Board([[Block::Empty; GRID_SIZE]; GRID_SIZE]);
        assert_eq!(board, Board::new());
    }

    #[test]
    fn in_row_validation_works() {
        assert_eq!(SAMPLE.is_number_in_row(7, 0), true);
        assert_eq!(SAMPLE.is_number_in_row(7, 8), true);
        assert_eq!(SAMPLE.is_number_in_row(9, 8), false);
    }

    #[test]
    fn in_column_validation_works() {
        assert_eq!(SAMPLE.is_number_in_column(9, 1), true);
        assert_eq!(SAMPLE.is_number_in_column(8, 1), false);
        assert_eq!(SAMPLE.is_number_in_column(5, 7), true);
        assert_eq!(SAMPLE.is_number_in_column(8, 7), false);
    }

    #[test]
    fn in_box_validation_works() {
        assert_eq!(SAMPLE.is_number_in_box(7, 1, 1), true);
        assert_eq!(SAMPLE.is_number_in_box(8, 1, 1), false);
        assert_eq!(SAMPLE.is_number_in_box(5, 8, 8), true);
        assert_eq!(SAMPLE.is_number_in_box(1, 8, 8), false);
    }

    #[test]
    fn display_test() {
        let board = SAMPLE;
        eprint!("{board}");
    }

    #[test]
    fn solve_works() {
        let mut board: Board = Board::from([
            [0, 1, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 7, 2, 0, 0],
            [5, 0, 8, 9, 0, 0, 0, 0, 4],
            [0, 0, 0, 0, 9, 0, 0, 5, 0],
            [4, 0, 6, 5, 0, 0, 0, 0, 8],
            [3, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 3, 0, 6, 0, 0, 0, 0, 0],
            [6, 0, 1, 0, 2, 0, 4, 0, 0],
            [0, 9, 0, 0, 0, 0, 0, 0, 1],
        ]);
        eprint!("{board}\n---------\n\n");
        if board.solve() {
            eprint!("{board}");
        } else {
            eprint!("Board could not be solved!");
        }
    }

    #[test]
    fn from_str_works() {
        let board = Board::from_str([
            "702050600",
            "000003000",
            "100009500",
            "800000090",
            "043000750",
            "090000008",
            "009700005",
            "000200000",
            "007040203",
        ]);
        assert_eq!(board, SAMPLE);
    }
}
