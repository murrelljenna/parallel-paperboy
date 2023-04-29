use bevy::prelude::*;
use bevy::math::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const ROAD_COLOR: Color = Color::rgb(0., 0., 0.);
const PATH_COLOR: Color = Color::rgb(1.0, 0.2, 0.2);
const PAPERBOY_COLOR: Color = Color::rgb(0.2, 0.2, 1.0);
const PAPERBOY_HIGHLIGHT_COLOR: Color = Color::rgb(0.2, 0.9, 0.2);
const TEXT_COLOR: Color = Color::rgb(0., 0., 0.);
const HOUSE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const ORIGIN_COLOR: Color = Color::rgb(0., 0., 0.);

#[derive(Component, Debug)]
struct Destination(bool);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_system(setup_drawing_map)
        .add_startup_system(add_potential_destinations)
        .add_system(activate_new_destination)
        .add_startup_system(add_people)
        .add_system(hello_world)
        .add_system(greet_people)
        .run();
}



fn add_potential_destinations(mut commands: Commands) {
    commands.spawn(
        Destination(false)
    ).insert(Transform::from_xyz(1f32, 1f32, 0f32));
}

#[derive(Component)]
struct Paperboy;

#[derive(Component)]
struct House;

#[derive(Component)]
struct Road;

#[derive(Component)]
struct Path;

fn setup_drawing_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}


fn hello_world() {
    println!("hello world!");
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