use crate::constants::*;
use bevy::{prelude::*, sprite::Anchor};

#[derive(Bundle)]
pub struct Wall {
    sprite: Sprite,
    transform: Transform,
}

impl Wall {
    pub fn new(location: WallLocation) -> Self {
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
        }
    }
}

pub enum WallLocation {
    Left,
    Right,
    Top,
    Bottom,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(0., 0.),
            WallLocation::Right => Vec2::new(ARENA_SIZE.x, 0.0),
            WallLocation::Top => Vec2::new(0.0, ARENA_SIZE.y),
            WallLocation::Bottom => Vec2::new(0.0, 0.),
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
