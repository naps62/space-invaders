use std::time::Duration;

use crate::constants::*;
use bevy::{prelude::*, time::common_conditions::on_timer};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                move_player_projectiles,
                move_enemy_projectiles.run_if(on_timer(Duration::from_secs_f32(3. / 60.))),
            ),
        );
    }
}

pub fn spawn_player_projectiles(mut cmds: Commands, assets: Res<AssetServer>, position: Vec2) {
    let projectile = assets.load("projectiles/player.png");
    let mut sprite = Sprite::from_image(projectile);
    sprite.custom_size = Some(Vec2::new(1., 4.));
    cmds.spawn((
        sprite,
        Transform::from_xyz(position.x, position.y, 0.0),
        PlayerProjectile,
    ));
}

pub fn spawn_enemy_projectiles(mut cmds: Commands, assets: Res<AssetServer>, position: Vec2) {
    let projectile = assets.load("projectiles/enemy_a.png");
    let mut sprite = Sprite::from_image(projectile);
    sprite.custom_size = Some(Vec2::new(1., 4.));
    cmds.spawn((
        sprite,
        Transform::from_xyz(position.x, position.y, 0.0),
        EnemyProjectile,
    ));
}

#[derive(Component)]
struct PlayerProjectile;

fn move_player_projectiles(mut projectiles: Query<&mut Transform, With<PlayerProjectile>>) {
    for mut projectile in projectiles.iter_mut() {
        projectile.translation.y += PLAYER_PROJECTILE_SPEED;
    }
}

#[derive(Component)]
struct EnemyProjectile;

fn move_enemy_projectiles(mut projectiles: Query<&mut Transform, With<EnemyProjectile>>) {
    for mut projectile in projectiles.iter_mut() {
        projectile.translation.y -= ENEMY_PROJECTILE_SPEED;
    }
}
