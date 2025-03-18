use crate::{
    constants::*,
    projectiles::{self},
};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .insert_resource(PlayerShootTimer::default())
            .add_systems(FixedUpdate, (move_player, player_shoot));
    }
}

fn startup(mut cmds: Commands, assets: Res<AssetServer>) {
    let player_sprite = assets.load("player.png");
    cmds.spawn((
        Player,
        Sprite {
            image: player_sprite,
            custom_size: Some(PLAYER_SIZE),
            color: Color::srgb(0., 1., 0.),
            ..default()
        },
        Transform::from_xyz(
            ARENA_SIZE.x / 2. - PLAYER_SIZE.x / 2.,
            PLAYER_FLOOR_GAP,
            0.0,
        ),
        projectiles::Collider,
    ));
}

#[derive(Component)]
pub struct Player;

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
    let left_bound = PLAYER_SIZE.x / 2.0 + PLAYER_PADDING;
    let right_bound = ARENA_SIZE.x - PLAYER_SIZE.x / 2.0 - PLAYER_PADDING;
    player.translation.x = new_position.clamp(left_bound, right_bound);
}

#[derive(Resource)]
pub struct PlayerShootTimer(Timer);

impl Default for PlayerShootTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Once))
    }
}

fn player_shoot(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Single<&mut Transform, With<Player>>,
    assets: Res<AssetServer>,
    mut timer: ResMut<PlayerShootTimer>,
    cmds: Commands,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() && keyboard.just_pressed(KeyCode::Space) {
        timer.0.reset();
        projectiles::spawn_player_projectiles(
            cmds,
            assets,
            Vec2::new(
                player.translation.x,
                player.translation.y + PLAYER_SIZE.y / 2.,
            ),
        );
    }
}
