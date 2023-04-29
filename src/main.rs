use bevy::prelude::*;
use bevy::math::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_potential_destinations)
        .add_system(activate_new_destination)
        .run();
}

#[derive(Component, Debug)]
struct Destination(bool);

fn add_potential_destinations(mut commands: Commands) {
    commands.spawn(
        Destination(false)
    ).insert(Transform::from_xyz(1f32, 1f32, 0f32));
}

// Make this run on a clock
// Make this pick one at random
// Add random variation to clock

fn activate_new_destination(mut query: Query<&mut Destination, With<Transform>>) {
    for mut destination in query.iter_mut() {
        destination.0 = true;
        println!("{:?}", destination)
    }
}