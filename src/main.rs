use std::time::Duration;
use bevy::prelude::*;
use bevy::math::*;
use rand::Rng;
mod graph;

const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const HOUSE_SIZE: Vec2 = Vec2::new(10., 10.);
const HORIZONTAL_ROAD_SIZE: Vec2 = Vec2::new(100., 10.);
const VERTICAL_ROAD_SIZE: Vec2 = Vec2::new(10., 100.);
const GAP_BETWEEN_HOUSE_AND_ROAD: f32 = 60.;

const GAP_BETWEEN_HOUSES: f32 = 60.0;
const GAP_BETWEEN_HOUSES_AND_CEILING: f32 = 10.0;
const GAP_BETWEEN_HOUSES_AND_SIDES: f32 = 20.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const ROAD_COLOR: Color = Color::rgb(0., 0., 0.);
//const PATH_COLOR: Color = Color::rgb(1.0, 0.2, 0.2);
//const PAPERBOY_COLOR: Color = Color::rgb(0.2, 0.2, 1.0);
//const PAPERBOY_HIGHLIGHT_COLOR: Color = Color::rgb(0.2, 0.9, 0.2);
//const TEXT_COLOR: Color = Color::rgb(0., 0., 0.);
const HOUSE_COLOR: Color = Color::rgb(0.84, 0.13, 0.13);
//const ORIGIN_COLOR: Color = Color::rgb(0., 0., 0.);
const WALL_COLOR: Color = Color::rgb(0., 0., 0.);

#[derive(Component, Debug)]
struct Destination(bool);

#[derive(Resource)]
struct NewDeliveryTimer(Timer);

const BASE_TIMER: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource::<graph::GameWorld>(graph::create_graph())
        .insert_resource(NewDeliveryTimer(Timer::from_seconds(BASE_TIMER, TimerMode::Repeating)))
        .add_startup_system(setup_drawing_map)
        .add_system(activate_new_destination)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn random_position() -> Vec2 {
    let xPosOrNeg: f32 = rand::thread_rng().gen();
    let yPosOrNeg: f32 = rand::thread_rng().gen();
    let xRng: f32 = rand::thread_rng().gen();
    let yRng: f32 = rand::thread_rng().gen();

    let mut x: f32;
    let mut y: f32;

    if (xPosOrNeg > 0.5) {
        x = xRng * 10f32;
    } else {
        x = xRng * -10f32;
    }

    if (yPosOrNeg > 0.5) {
        y = yRng * 10f32;
    } else {
        y = yRng * -10f32;
    }

    return Vec2::new(x, y)
}

fn activate_new_destination(
    time: Res<Time>, mut timer: ResMut<NewDeliveryTimer>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        let x: f32 = rand::thread_rng().gen();
        let new_duration = Duration::from_secs_f32(BASE_TIMER * (x * 2.));
        timer.0.set_duration(new_duration);
        println!("hello {}!", random_position());
    }
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Paperboy;

#[derive(Component)]
struct House;

#[derive(Component)]
struct Road;

#[derive(Component)]
struct Path;

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

const SCALEUP_FACTOR: f32 = 40. as f32;
const ROAD_THICKNESS: f32 = 20. as f32;

fn setup_drawing_map(
    mut commands: Commands,
    map: Res<graph::GameWorld>
) {

    // Camera
    commands.spawn(Camera2dBundle::default());

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));

    let total_width_of_map = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_HOUSES_AND_SIDES;
    let total_height_of_map = (TOP_WALL - BOTTOM_WALL) - 2. * GAP_BETWEEN_HOUSES_AND_CEILING;

    assert!(total_width_of_map > 0.0);
    assert!(total_height_of_map > 0.0);
    println!("width of map: {:?}", total_width_of_map);
    println!("height of map: {:?}", total_height_of_map);


    for edge_index in map.graph.edge_indices() {
      println!("edge: {:?}", edge_index);
      if let Some(endpoints) = map.graph.edge_endpoints(edge_index) {
        let (start_index, end_index) = endpoints;
        let start_pos = map.graph.node_weight(start_index).unwrap().pos;
        let end_pos = map.graph.node_weight(end_index).unwrap().pos;
        println!("start_pos: {:?}", start_pos);
        println!("end_pos: {:?}", end_pos);
        let offset = Vec2::new(total_width_of_map/4., total_height_of_map/2.);
        let road_position = Vec2::new(
            (start_pos.x*SCALEUP_FACTOR+end_pos.x*SCALEUP_FACTOR)/2.-offset.x,
            (start_pos.y*SCALEUP_FACTOR+end_pos.y*SCALEUP_FACTOR)/2.);
        println!("road_position: {:?}", road_position);

        let road_scale = Vec3::new((start_pos.x*SCALEUP_FACTOR-end_pos.x*SCALEUP_FACTOR+ROAD_THICKNESS), (start_pos.y*SCALEUP_FACTOR-end_pos.y*SCALEUP_FACTOR+ROAD_THICKNESS), 1.0);
        println!("road_scale: {:?}", road_scale);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: ROAD_COLOR,
                    ..default()
                },
                transform: Transform {
                    translation: road_position.extend(0.0),
                    scale: road_scale,
                    ..default()
                },
                ..default()
            },
            Road,
            Collider,
        ));
      }
    }
}