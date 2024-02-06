use crate::board::Board;

// pub struct Piece {
//     symbol: char,
//     board: Board,
//     pos: [u8;2],
// }     

// maybe make Pieces an enum?

struct Pawn {
    symbol: char,
    board: Board,
    pos: [u8;2], 
}

struct King {
    symbol: char,
    board: Board,
    pos: [u8;2], 
}

struct Queen {
    symbol: char,
    board: Board,
    pos: [u8;2], 
}

struct Bishop {
    symbol: char,
    board: Board,
    pos: [u8;2], 
}

struct Knight {
    symbol: char,
    board: Board,
    pos: [u8;2], 
}

struct Rook  {
    symbol: char,
    board: Board,
    pos: [u8;2], 
}

enum Piece {
    PawnPiece(Pawn),
    KingPiece(King),
    QueenPiece(Queen).
    BishopPiece(Bishop),
    KnightPiece(Knight),
    RookPiece(Rook),
}