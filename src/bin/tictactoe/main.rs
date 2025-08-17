

use abmixmax::{Algorithm};
mod board;
use board::Board;

fn main() {
    let board = Board::new(2,3,2);
    println!("{:?}", &board);
    let board2 = board.apply_move(1);
    println!("{:?}", &board2);
        let board3 = board.apply_move(7);
    println!("{:?}", &board3);

}