use bevy::{color::Color, math::Vec2};

pub const BG_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const PLAYER_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
pub const WALL_COLOR: Color = Color::srgb(0., 0., 0.);
pub const PLAYER_SPEED: f32 = 3.0;

pub const WALL_THICKNESS: f32 = 1000.0;
pub const ARENA_SIZE: Vec2 = Vec2::new(224.0, 256.0);

pub const PLAYER_SIZE: Vec2 = Vec2::new(24.0, 16.0);
pub const PLAYER_FLOOR_GAP: f32 = 20.0;
pub const PLAYER_PADDING: f32 = 0.0;

pub const ENEMY_COLOR: Color = Color::srgb(0.7, 0.3, 0.3);
pub const ENEMY_SIZE: Vec2 = Vec2::new(24.0, 16.0);
pub const ENEMY_SPACING: f32 = 10.0;
pub const ENEMY_WALL_GAP: f32 = 10.0;
