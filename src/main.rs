use bevy::prelude::*;
use bevy::math::*;
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup_drawing_map)
        .add_startup_system(add_potential_destinations)
        .add_startup_system(graph::create_graph)
        .add_system(activate_new_destination)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn add_potential_destinations(mut commands: Commands) {
    commands.spawn(
        Destination(false)
    ).insert(Transform::from_xyz(1., 1., 0.));
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

fn setup_drawing_map(
    mut commands: Commands,
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

    // Given the space available, compute how many rows and columns of bricks we can fit
    let n_columns = (total_width_of_map / (HOUSE_SIZE.x + GAP_BETWEEN_HOUSES)).floor() as usize;
    let n_rows = (total_height_of_map / (HOUSE_SIZE.y + GAP_BETWEEN_HOUSES)).floor() as usize;

    assert!(n_columns > 0);
    assert!(n_rows > 0);

    // Because we need to round the number of columns,
    // the space on the top and sides of the bricks only captures a lower bound, not an exact value
    let horizontal_center_of_map = (LEFT_WALL + RIGHT_WALL) / 2.0;
    let left_edge_of_houses = horizontal_center_of_map
        // Space taken up by the bricks
        - (n_columns as f32 / 2.0 * HOUSE_SIZE.x)
        // Space taken up by the gaps
        - (n_columns - 1) as f32 / 2.0 * GAP_BETWEEN_HOUSES;

    let vertical_center_of_map = (TOP_WALL + BOTTOM_WALL) / 2.0;
    let bottom_edge_of_houses = vertical_center_of_map
        // Space taken up by the bricks
        - (n_rows as f32 / 2.0 * HOUSE_SIZE.y)
        // Space taken up by the gaps
        - (n_rows - 1) as f32 / 2.0 * GAP_BETWEEN_HOUSES;

    // In Bevy, the `translation` of an entity describes the center point,
    // not its bottom-left corner
    let offset_x = left_edge_of_houses + HOUSE_SIZE.x / 2.;
    let offset_y = bottom_edge_of_houses + HOUSE_SIZE.y / 2.;

    for row in 0..n_rows {
        for column in 0..n_columns {
            let house_position = Vec2::new(
                offset_x + column as f32 * (HOUSE_SIZE.x + GAP_BETWEEN_HOUSES),
                offset_y + row as f32 * (HOUSE_SIZE.y + GAP_BETWEEN_HOUSES),
            );

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: HOUSE_COLOR,
                        ..default()
                    },
                    transform: Transform {
                        translation: house_position.extend(0.0),
                        scale: Vec3::new(HOUSE_SIZE.x, HOUSE_SIZE.y, 1.0),
                        ..default()
                    },
                    ..default()
                },
                House,
                Collider,
            ));
        }
    }

    for row in 0..n_rows - 1 {
        for column in 0..n_columns - 1 {
            let road_position = Vec2::new(
                offset_x + 37.5 + column as f32 * (HOUSE_SIZE.x + GAP_BETWEEN_HOUSE_AND_ROAD),
                offset_y + 37.5 + row as f32 * (HOUSE_SIZE.y + GAP_BETWEEN_HOUSE_AND_ROAD),
            );

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: ROAD_COLOR,
                        ..default()
                    },
                    transform: Transform {
                        translation: road_position.extend(0.0),
                        scale: Vec3::new(HORIZONTAL_ROAD_SIZE.x, HORIZONTAL_ROAD_SIZE.y, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Road,
                Collider,
            ));
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: ROAD_COLOR,
                        ..default()
                    },
                    transform: Transform {
                        translation: road_position.extend(0.0),
                        scale: Vec3::new(VERTICAL_ROAD_SIZE.x, VERTICAL_ROAD_SIZE.y, 1.0),
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