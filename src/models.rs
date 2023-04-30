use bevy::prelude::*;
use bevy::math::*;

struct Path(Vec<Vec2>);


struct Paperboy();

impl Paperboy {
    fn start(self, path: Path) {}
}

#[derive(Component, Debug)]
pub struct House {
    pub active: bool
}

const HOUSE_COLOR: Color = Color::rgb(0., 0., 0.);

pub fn initialize_houses(mut commands: Commands) {
    let mut HOUSE_POSITIONS: Vec<Vec2> = Vec::new();

    // HOUSE POSITIONS

    HOUSE_POSITIONS.push(vec2(100.0, 115.0));
    HOUSE_POSITIONS.push(vec2(45.0, 115.0));
    HOUSE_POSITIONS.push(vec2(-10.0, 115.0));

    HOUSE_POSITIONS.push(vec2(100.0, 45.0));
    HOUSE_POSITIONS.push(vec2(45.0, 45.0));
    HOUSE_POSITIONS.push(vec2(-10.0, 45.0));

    let scale = Vec3::new(45.0, 60.0, 0.0);
    for pos in HOUSE_POSITIONS {
        commands.spawn((
            House { active: false },
            SpriteBundle {
                sprite: Sprite {
                    color: HOUSE_COLOR,
                    ..default()
                },
                transform: Transform {
                    translation: pos.extend(0.0),
                    scale: scale,
                    ..default()
                },
                ..default()
            }));
    }
}