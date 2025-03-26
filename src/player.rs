use crate::{
    constants::*,
    shots::{self, Hit, PlayerShot},
    GameState,
};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Lives(3))
            .add_systems(OnEnter(GameState::Playing), startup)
            .add_systems(
                FixedUpdate,
                (move_player, player_shoot).run_if(in_state(GameState::Playing)),
            );
    }
}

fn startup(mut cmds: Commands, assets: Res<AssetServer>) {
    let player_sprite = assets.load("sprites/player.png");
    cmds.spawn((
        Player,
        Sprite {
            image: player_sprite,
            custom_size: Some(PLAYER_SIZE),
            color: Color::srgb(0., 1., 0.),
            ..default()
        },
        Transform::from_xyz(ARENA_SIZE.x / 2., PLAYER_FLOOR_GAP, 0.0),
        shots::Collider::player_layer(),
    ))
    .observe(on_hit);
}

fn on_hit(_trigger: Trigger<Hit>, mut lives: ResMut<Lives>, mut _cmds: Commands) {
    lives.0 -= 1;
}

#[derive(Resource, Default)]
pub struct Lives(pub usize);

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

fn player_shoot(
    has_shot: Query<(), With<PlayerShot>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Single<&mut Transform, With<Player>>,
    assets: Res<AssetServer>,
    cmds: Commands,
) {
    if has_shot.is_empty() && keyboard.just_pressed(KeyCode::Space) {
        shots::spawn_player_shots(
            cmds,
            assets,
            Vec2::new(
                player.translation.x,
                player.translation.y + PLAYER_SIZE.y / 2.,
            ),
        );
    }
}
