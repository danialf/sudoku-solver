use std::fmt::Display;

const GRID_SIZE: usize = 9;

#[derive(PartialEq, Debug)]
pub struct Board([[u8; GRID_SIZE]; GRID_SIZE]);

impl Board {
    pub fn new() -> Self {
        Board([[0; GRID_SIZE]; GRID_SIZE])
    }

    pub fn is_number_in_row(&self, number: u8, row: u8) -> bool {
        for i in 0..GRID_SIZE {
            if self.0[row as usize][i] == number {
                return true;
            }
        }
        false
    }

    pub fn is_number_in_column(&self, number: u8, column: u8) -> bool {
        for i in 0..GRID_SIZE {
            if self.0[i][column as usize] == number {
                return true;
            }
        }
        false
    }

    pub fn is_number_in_box(&self, number: u8, row: u8, column: u8) -> bool {
        let local_box_row = row - row % 3;
        let local_box_column = column - column % 3;

        for i in local_box_row..(local_box_row + 3) {
            for j in local_box_column..(local_box_column + 3) {
                if self.0[i as usize][j as usize] == number {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_valid_placement(&self, number: u8, row: u8, column: u8) -> bool {
        !self.is_number_in_row(number, row)
            && !self.is_number_in_column(number, column)
            && !self.is_number_in_box(number, row, column)
    }

    pub fn solve_board(&mut self) -> bool {
        for row in 0..GRID_SIZE {
            for column in 0..GRID_SIZE {
                if self.0[row][column] == 0 {
                    for number_to_try in 1..=GRID_SIZE {
                        if self.is_valid_placement(number_to_try as u8, row as u8, column as u8) {
                            self.0[row][column] = number_to_try as u8;

                            if self.solve_board() {
                                return true;
                            } else {
                                self.0[row][column] = 0;
                            }
                        }
                    }
                    return false;
                }
            }
        }
        true
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let _ = if j % 3 == 0 {
                    write!(f, "║")
                } else {
                    write!(f, "|")
                };
                let _ = write!(f, " {} ", self.0[i][j]);
            }
            let _ = writeln!(f, "║");
            // if i == 0{
            //     continue;
            // }

            // let _ = if i-1 % 3 == 0 {
            //     writeln!(f, "{}", String::from("═").repeat(37))
            // } else {
            //     writeln!(f, "{}", String::from("-").repeat(37))
            // };
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Board;
    const SAMPLE: Board = Board([
        [7, 0, 2, 0, 5, 0, 6, 0, 0],
        [0, 0, 0, 0, 0, 3, 0, 0, 0],
        [1, 0, 0, 0, 0, 9, 5, 0, 0],
        [8, 0, 0, 0, 0, 0, 0, 9, 0],
        [0, 4, 3, 0, 0, 0, 7, 5, 0],
        [0, 9, 0, 0, 0, 0, 0, 0, 8],
        [0, 0, 9, 7, 0, 0, 0, 0, 5],
        [0, 0, 0, 2, 0, 0, 0, 0, 0],
        [0, 0, 7, 0, 4, 0, 2, 0, 3],
    ]);

    #[test]
    fn fill_board() {
        let board = Board([
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]);
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
        let mut board: Board = Board([
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
        if board.solve_board() {
            eprint!("{board}");
        } else {
            eprint!("Board could not be solved!");
        }
    }
}
