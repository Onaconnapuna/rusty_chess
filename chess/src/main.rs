pub mod board;
pub mod piece;
// how to avoid borrow checker problems

// Board needs to implement Board.move_piece(&self, start pos, end pos)
// with only one mut reference what to we do with pieces.
// how about all the pieces will hold a read reference to the board
// the board will always change itself within its own function impl

/*fn main() {
    let mut _board = board::Board {
       grid: [ [Piece { kind: PieceKind::Pawn, color: PieceColor::White }; 8]; 8]
    };

    // for row in _board.grid {
    //     println!("Row: {:?}", row);
    // }
} */
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

struct Entity(u64);

use bevy::prelude::*;

fn hello_world() {
    println!("hello world!");
}

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, hello_world)
        .run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Connor Germain".to_string())));
    commands.spawn((Person, Name("Barak Obama".to_string())));
}
