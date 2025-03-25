use bevy::{color::Color, math::Vec2};

pub const BG_COLOR: Color = Color::srgb(0., 0., 0.);
pub const GREEN: Color = Color::srgb(0., 1., 0.);

pub const WALL_THICKNESS: f32 = 1.0;
pub const ARENA_SIZE: Vec2 = Vec2::new(224.0, 256.0);

pub const PLAYER_SIZE: Vec2 = Vec2::new(12.0, 8.0);
pub const PLAYER_FLOOR_GAP: f32 = 45.0;
pub const PLAYER_PADDING: f32 = 0.0;
pub const PLAYER_PROJECTILE_SPEED: f32 = 3.0;
pub const PLAYER_SPEED: f32 = 1.0;

pub const ENEMY_SIZE: Vec2 = Vec2::splat(16.);
pub const ENEMY_SPACING: f32 = 3.0;
pub const ENEMY_WALL_GAP: f32 = 10.0;
pub const ENEMY_PROJECTILE_SPEED: f32 = 3.0;
