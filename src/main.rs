mod constants;
mod wall;

use bevy::{prelude::*, render::camera::ScalingMode};
use constants::*;
use wall::{Wall, WallLocation};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space Invaders".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BG_COLOR))
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, move_player)
        .run();
}

fn startup(mut cmds: Commands) {
    // camera
    cmds.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: RIGHT_WALL - LEFT_WALL,
                min_height: TOP_WALL - BOTTOM_WALL,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    // walls
    cmds.spawn(Wall::new(WallLocation::Left));
    cmds.spawn(Wall::new(WallLocation::Right));
    cmds.spawn(Wall::new(WallLocation::Top));
    cmds.spawn(Wall::new(WallLocation::Bottom));

    // player
    cmds.spawn((
        Player,
        Sprite {
            color: PLAYER_COLOR,
            custom_size: Some(PLAYER_SIZE),
            ..default()
        },
        Transform::from_xyz(0.0, BOTTOM_WALL + PLAYER_FLOOR_GAP, 0.0),
    ));
}

#[derive(Component)]
struct Player;

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let mut direction = 0.0;

    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        direction -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        direction += 1.0;
    }

    let new_position = player.translation.x + direction * PLAYER_SPEED;
    let left_bound = LEFT_WALL + PLAYER_SIZE.x / 2.0 + PLAYER_PADDING;
    let right_bound = RIGHT_WALL - PLAYER_SIZE.x / 2.0 - PLAYER_PADDING;
    player.translation.x = new_position.clamp(left_bound, right_bound);
}
