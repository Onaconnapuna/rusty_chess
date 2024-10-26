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

#[derive(Resource)]
struct GreetTimer(Timer);

struct Entity(u64);

use bevy::prelude::*;

fn hello_world() {
    println!("hello world!");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Connor Germain".to_string())));
    commands.spawn((Person, Name("Barak Obama".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Connor Germain" {
            name.0 == "Connor St. Germain".to_string();
            break;
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}
