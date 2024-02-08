use crate::board::Board;

#[derive(Copy, Clone)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: PieceColor,
}

#[derive(Copy, Clone)]
pub enum PieceKind {
    Pawn, 
    Knight, 
    Bishop, 
    Rook, 
    Queen,
    King,
}

#[derive(Copy, Clone)]
pub enum PieceColor {
    White, 
    Black,
}