mod board;
mod piece; 

fn main() {
    let mut _board = board::Board {
       grid: [ [0; 8]; 8]
    };

    for row in _board.grid {
        println!("Row: {:?}", row);
    }
}