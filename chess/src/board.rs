// mutability of a struct is in its binding, not declaration, so when I create an instance, thats when I use mut;
use crate::piece::Piece;

pub struct Board {
    pub grid: [ [Option<Piece>; 8]; 8],
}

impl Board {
    fn move_piece() {}
}