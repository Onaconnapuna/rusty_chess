pub mod board;
pub mod piece;
// how to avoid borrow checker problems

// Board needs to implement Board.move_piece(&self, start pos, end pos)
// with only one mut reference what to we do with pieces.
// how about all the pieces will hold a read reference to the board
// the board will always change itself within its own function impl 

fn main() {
    let mut _board = board::Board {
       grid: [ [Piece { kind: PieceKind::Pawn, color: PieceColor::White }; 8]; 8]
    };

    // for row in _board.grid {
    //     println!("Row: {:?}", row);
    // }
}