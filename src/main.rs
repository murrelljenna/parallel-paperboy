use std::time::Duration;
use bevy::prelude::*;
use bevy::math::*;
use rand::*;
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use bevy::input::mouse::MouseButtonInput;
use bevy::window::PrimaryWindow;
use bevy::render::camera::RenderTarget;
mod graph;
mod models;

const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

//const HOUSE_SIZE: Vec2 = Vec2::new(10., 10.);
const PAPERBOY_SIZE: Vec2 = Vec2::new(10., 10.);

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const ROAD_COLOR: Color = Color::rgb(0., 0., 0.);
const PATH_COLOR: Color = Color::rgb(1.0, 0.2, 0.2);
const PAPERBOY_COLOR: Color = Color::rgb(0.2, 0.2, 1.0);
//const PAPERBOY_HIGHLIGHT_COLOR: Color = Color::rgb(0.2, 0.9, 0.2);
//const TEXT_COLOR: Color = Color::rgb(0., 0., 0.);
//const HOUSE_COLOR: Color = Color::rgb(0.84, 0.13, 0.13);
//const ORIGIN_COLOR: Color = Color::rgb(0., 0., 0.);
const WALL_COLOR: Color = Color::rgb(0., 0., 0.);

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
        .add_startup_system(models::initialize_houses)
        .add_system(activate_new_destination)
        .add_system(delivery_command)
        .add_system(mouse_button_place_paperboy)
        .add_system(mouse_button_place_path)
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
    time: Res<Time>, mut timer: ResMut<NewDeliveryTimer>, mut query: Query<&mut models::House, With<Transform>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng: ThreadRng = rand::thread_rng();
        let x: f32 = rng.gen();
        let new_duration = Duration::from_secs_f32(BASE_TIMER * (x * 2.));
        timer.0.set_duration(new_duration);

        if let Some(mut house) = query.iter_mut().choose(&mut rng) {
            house.active = true;
        }
    }
}

#[derive(Debug, PartialEq)]
enum SelectionMode {
    PlacingPaperboy,
    PlacingPath,
    Paused,
}

#[derive(Component)]
struct UIState {
    selection_mode: SelectionMode,
}

impl UIState {
    fn new() -> UIState {
        UIState { selection_mode: SelectionMode::PlacingPaperboy }
    }
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Paperboy;

#[derive(Component)]
struct Road;

#[derive(Component, Debug)]
struct Path {
    points: Vec<Vec2>,
    entities: Vec<Entity>,
}

impl Path {
    fn new() -> Path {
        Path { points: vec![], entities: vec![] }
    }
}

#[derive(Component, Debug)]
struct PathSegment;

// #[derive(Component, Debug)]
// struct PathHolder {
//     paths: Vec<Path>,
// }

// impl PathHolder {
//     fn new() -> PathHolder {
//         PathHolder { paths: vec![] }
//     }
// }

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

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

fn mouse_button_place_path(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    ui_state: Query<&UIState>,
    mut paths: Query<&mut Path>,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
) {
    if ui_state.single().selection_mode != SelectionMode::PlacingPath {
        // this method doesn't run in that mode
        return
    }

    use bevy::input::ButtonState;

    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = if let RenderTarget::Window(_id) = camera.target {
        windows.single()
    } else {
        windows.single()
    };

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                println!("placing path!!!!");
                println!("Mouse button press: {:?}", ev.button);
                println!("Mouse button state: {:?}", ev.state);
                // check if the cursor is inside the window and get its position
                // then, ask bevy to convert into world coordinates, and truncate to discard Z
                if let Some(world_position) = window.cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())
                {
                    eprintln!("World coords: {}/{}", world_position.x, world_position.y);
                    for mut path in &mut paths {
                        let len = path.points.len();
                        if len >= 1 {
                            let path_position = Vec2::new((path.points[len-1].x+world_position.x)/2., (path.points[len-1].y+world_position.y)/2.);
                            let xlen = path.points[len-1].x-world_position.x;
                            let ylen = path.points[len-1].y-world_position.y;
                            let pythagorean_len = (xlen*xlen+ylen*ylen).sqrt();
                            let path_scale = Vec2::new(pythagorean_len, ROAD_THICKNESS);
                            let path_sides_ratio = (path.points[len-1].y-world_position.y).atan2(
                                    path.points[len-1].x-world_position.x
                            );
                            println!("path.points[len-1]: {:?}", path.points[len-1]);
                            println!("path position: {:?}", path_position);
                            println!("path scale: {:?}", path_scale);
                            println!("path sides ratio: {:?}", path_sides_ratio);
                            path.entities.push(
                                commands.spawn((
                                    SpriteBundle {
                                        sprite: Sprite {
                                            color: PATH_COLOR,
                                            ..default()
                                        },
                                        transform: Transform {
                                            translation: path_position.extend(0.0),
                                            scale: path_scale.extend(0.0),
                                            rotation: Quat::from_rotation_z(path_sides_ratio),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    PathSegment,
                                    Collider,
                                )).id()
                            );
                        }
                        path.points.push(Vec2::new(world_position.x, world_position.y));
                        println!("path points: {:?}", path.points);
                    }
                }
            },
            ButtonState::Released => {
                println!("Mouse button release (path branch): {:?}", ev.button);
            }
        }
    }
}

fn mouse_button_place_paperboy(
    // need to get window dimensions
    windows: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut paperboy_transform: Query<&mut Transform, With<Paperboy>>,
    ui_state: Query<&UIState>,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
) {
    if ui_state.single().selection_mode != SelectionMode::PlacingPaperboy {
        // this method doesn't run in that mode
        return
    }

    use bevy::input::ButtonState;

    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = if let RenderTarget::Window(_id) = camera.target {
        windows.single()
    } else {
        windows.single()
    };

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                println!("placing paperboy!!!!");
                println!("Mouse button press: {:?}", ev.button);
                println!("Mouse button state: {:?}", ev.state);

                // check if the cursor is inside the window and get its position
                // then, ask bevy to convert into world coordinates, and truncate to discard Z
                if let Some(world_position) = window.cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())
                {
                    eprintln!("World coords: {}/{}", world_position.x, world_position.y);
                    for mut transform in &mut paperboy_transform {
                        transform.translation.x = world_position.x;
                        transform.translation.y = world_position.y;
                    }
                }
            }
            ButtonState::Released => {
                println!("Mouse button release (pboy branch): {:?}", ev.button);
            }
        }
    }
}

fn delivery_command(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    paperboy_transform: Query<&Transform, With<Paperboy>>,
    mut ui_states: Query<&mut UIState>,
    mut paths: Query<&mut Path>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for transform in &paperboy_transform {
            println!("space pressed, paperboy at {:?}", transform);
        }
    } else if keys.just_pressed(KeyCode::Tab) {
        println!("tab pressed, UI state is {:?}", ui_states.single().selection_mode);
        for mut ui_state in &mut ui_states {
            ui_state.selection_mode =  match ui_state.selection_mode {
                SelectionMode::PlacingPaperboy => SelectionMode::PlacingPath,
                SelectionMode::PlacingPath => SelectionMode::PlacingPaperboy,
                SelectionMode::Paused => SelectionMode::Paused
            }
        }
    } else if keys.just_pressed(KeyCode::Q) {
        println!("q pressed, paths is {:?}", paths.single().points);
        for mut path in &mut paths {
            for entity in &path.entities {
                commands.entity(*entity).despawn()
            }
            path.points.clear();
            path.entities.clear();
        }
        println!("paths emptied, paths is now {:?}", paths.single().points);
    }
}


fn setup_drawing_map(
    mut commands: Commands,
    map: Res<graph::GameWorld>
) {
    // Path holder
    commands.spawn(Path::new());

    // UIState
    commands.spawn(UIState::new());

    // Camera
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));

    let total_width_of_map = RIGHT_WALL - LEFT_WALL;
    let total_height_of_map = TOP_WALL - BOTTOM_WALL;

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

        let road_scale = Vec3::new(start_pos.x*SCALEUP_FACTOR-end_pos.x*SCALEUP_FACTOR+ROAD_THICKNESS, start_pos.y*SCALEUP_FACTOR-end_pos.y*SCALEUP_FACTOR+ROAD_THICKNESS, 1.0);
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

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: PAPERBOY_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec2::new(0., 20.).extend(0.0),
                scale: PAPERBOY_SIZE.extend(0.0),
                ..default()
            },
            ..default()
        },
        Paperboy,
        Collider,
    ));
}