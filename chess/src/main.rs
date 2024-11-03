use bevy::prelude::*;

#[derive(Component)]
struct MyCameraMarker;

#[derive(Component)]
struct Position {
    x: i8,
    y: i8,
}

fn main() {
    App::new()
        .add_plugins(WindowPlugin {
            primary_window: Some(Window {
                resolution: (140.0, 140.0).into(),
                title: "Rusty Chess".to_string(),
                ..default()
            }),
            ..default()
        })
        .add_systems(Startup, camera_setup)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle { ..default() }, MyCameraMarker));
}

fn debug_projection(query_camera: Query<&OrthographicProjection, With<MyCameraMarker>>) {
    let projection = query_camera.single();
}
