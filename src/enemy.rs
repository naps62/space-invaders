use crate::constants::*;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    enemy: Enemy,
    sprite: Sprite,
    transform: Transform,
}

impl EnemyBundle {
    pub fn new(location: Vec2, size: Vec2, image: Handle<Image>) -> Self {
        Self {
            sprite: Sprite {
                image,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(location.extend(0.0)),
            ..default()
        }
    }
}

#[derive(Debug, Default, Resource, PartialEq, Eq)]
pub enum EnemyDirection {
    #[default]
    Right,
    Left,
}

#[derive(Event, Default)]
pub struct EnemyDirectionChanged;

impl EnemyDirection {
    pub fn reverse(&mut self) {
        if *self == EnemyDirection::Right {
            *self = EnemyDirection::Left;
        } else {
            *self = EnemyDirection::Right;
        }
    }

    pub fn as_f32(&self) -> f32 {
        match self {
            EnemyDirection::Right => 1.0,
            EnemyDirection::Left => -1.0,
        }
    }
}
