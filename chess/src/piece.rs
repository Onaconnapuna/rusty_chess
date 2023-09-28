use crate::board::Board;

pub struct Piece {
    symbol: char,
    board: Board,
    pos: [u8;2],
}     