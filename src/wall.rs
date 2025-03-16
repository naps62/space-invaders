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
                translation: location.position().extend(0.0),
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
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.0),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.0),
            WallLocation::Top => Vec2::new(0.0, TOP_WALL),
            WallLocation::Bottom => Vec2::new(0.0, BOTTOM_WALL),
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
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;

        match self {
            //WallLocation::Left => Vec2::new(WALL_THICKNESS, arena_height),
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Top | WallLocation::Bottom => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}
