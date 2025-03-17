mod constants;
mod enemy;
mod enemy_squad;
mod wall;

use std::time::Duration;

use bevy::{prelude::*, render::camera::ScalingMode, time::common_conditions::on_timer};
use constants::*;
use enemy::{Enemy, EnemyBundle, EnemyDirection, EnemyDirectionChanged};
use enemy_squad::{EnemySquad, EnemySquadBundle};
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
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(EnemyDirection::default())
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (move_enemies.run_if(on_timer(Duration::from_secs_f32(0.1))),),
        )
        .add_systems(FixedUpdate, (move_player, swap_enemy_direction))
        .run();
}

fn startup(mut cmds: Commands, assets: Res<AssetServer>) {
    // camera
    cmds.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: ARENA_SIZE.x,
                min_height: ARENA_SIZE.y,
            },
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(ARENA_SIZE.x / 2.0, ARENA_SIZE.y / 2.0, 0.0),
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
        Transform::from_xyz(
            ARENA_SIZE.x / 2. - PLAYER_SIZE.x / 2.,
            PLAYER_FLOOR_GAP,
            0.0,
        ),
    ));

    // enemies
    let enemy_sprite = assets.load("Invader_01-1.png");
    cmds.spawn(EnemySquadBundle::default())
        .with_children(|squad| {
            // starting position for enemies
            let enemy_start = Vec2::new(
                ENEMY_SIZE.x / 2.0 + ENEMY_WALL_GAP,
                -ENEMY_SIZE.y / 2.0 + ARENA_SIZE.y - ENEMY_WALL_GAP,
            );
            for y in 0..5 {
                let mut current_enemy_pos = enemy_start;
                current_enemy_pos.y -= (ENEMY_SIZE.y + ENEMY_SPACING) * y as f32;
                for _x in 0..5 {
                    squad.spawn(EnemyBundle::new(
                        current_enemy_pos,
                        ENEMY_SIZE,
                        enemy_sprite.clone(),
                    ));
                    current_enemy_pos.x += ENEMY_SIZE.x + ENEMY_SPACING;
                }
            }
        });
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
    let left_bound = -ARENA_SIZE.x / 2.0 + PLAYER_SIZE.x / 2.0 + PLAYER_PADDING;
    let right_bound = ARENA_SIZE.x / 2.0 - PLAYER_SIZE.x / 2.0 - PLAYER_PADDING;
    player.translation.x = new_position.clamp(left_bound, right_bound);
}

fn move_enemies(
    direction: Res<EnemyDirection>,
    mut squad: Single<&mut Transform, With<EnemySquad>>,
) {
    squad.translation.x += 10.0 * direction.as_f32();
}

fn swap_enemy_direction(
    current: ResMut<EnemyDirection>,
    enemies: Query<&GlobalTransform, With<Enemy>>,
    squad: Single<&mut Transform, With<EnemySquad>>,
) {
    let direction = current.into_inner();
    for enemy in enemies.iter() {
        let x = enemy.translation().x;
        let needs_reverse = match direction {
            EnemyDirection::Right => x + ENEMY_SIZE.x / 2. + ENEMY_WALL_GAP >= ARENA_SIZE.x,
            EnemyDirection::Left => x - ENEMY_SIZE.x / 2. - ENEMY_WALL_GAP <= 0.,
        };

        // change direction and lower
        if needs_reverse {
            direction.reverse();
            let mut squad = squad.into_inner();
            squad.translation.y -= 5.0;
            return;
        }
    }
}
