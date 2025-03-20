use crate::{constants::*, shots::Collider};
use bevy::{prelude::*, sprite::Anchor};

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup(mut cmds: Commands) {
    cmds.spawn(Wall::new(WallLocation::Left));
    cmds.spawn(Wall::new(WallLocation::Right));
    cmds.spawn(Wall::new(WallLocation::Top));
    cmds.spawn(Wall::new(WallLocation::Bottom));
}

#[derive(Bundle)]
struct Wall {
    sprite: Sprite,
    transform: Transform,
    collider: Collider,
}

impl Wall {
    fn new(location: WallLocation) -> Self {
        Self {
            sprite: Sprite {
                color: WALL_COLOR,
                anchor: location.anchor(),
                ..default()
            },
            transform: Transform {
                translation: location.position().extend(1.0),
                scale: location.size().extend(1.0),
                ..default()
            },
            collider: Collider::wall_layer(),
        }
    }
}

enum WallLocation {
    Left,
    Right,
    Top,
    Bottom,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(0., ARENA_SIZE.y / 2.),
            WallLocation::Right => Vec2::new(ARENA_SIZE.x, ARENA_SIZE.y / 2.),
            WallLocation::Top => Vec2::new(ARENA_SIZE.x / 2., ARENA_SIZE.y),
            WallLocation::Bottom => Vec2::new(ARENA_SIZE.x / 2., 0.),
        }
    }

    fn anchor(&self) -> Anchor {
        match self {
            WallLocation::Left => Anchor::CenterRight,
            WallLocation::Right => Anchor::CenterLeft,
            WallLocation::Top => Anchor::BottomCenter,
            WallLocation::Bottom => Anchor::TopCenter,
        }
    }

    fn size(&self) -> Vec2 {
        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, ARENA_SIZE.y + WALL_THICKNESS)
            }
            WallLocation::Top | WallLocation::Bottom => {
                Vec2::new(ARENA_SIZE.x + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}
