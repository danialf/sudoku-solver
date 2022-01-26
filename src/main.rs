use sudoku_solver::Board;
fn main() {
    let mut board = Board::from_str([
        "000002000",
        "080000007",
        "006310900",
        "060250080",
        "000004500",
        "002009000",
        "100000090",
        "000040000",
        "003560100",
    ]);
    println!("{board}");

    if board.solve() {
        println!("solved the board");
    } else {
        println!("cannot solve it");
    }

    println!("{board}");
}
