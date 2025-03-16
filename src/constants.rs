use bevy::{color::Color, math::Vec2};

pub const BG_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const PLAYER_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
pub const WALL_COLOR: Color = Color::srgb(0., 0., 0.);
pub const PLAYER_SPEED: f32 = 3.0;

pub const WALL_THICKNESS: f32 = 1000.0;
pub const LEFT_WALL: f32 = -112.0;
pub const RIGHT_WALL: f32 = 112.0;
pub const TOP_WALL: f32 = 128.0;
pub const BOTTOM_WALL: f32 = -128.0;

pub const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 5.0);
pub const PLAYER_FLOOR_GAP: f32 = 20.0;
pub const PLAYER_PADDING: f32 = 0.0;
