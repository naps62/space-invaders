use crate::constants::*;
use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (move_player_projectiles, move_enemy_projectiles),
        );
    }
}

#[derive(Component)]
pub struct PlayerProjectile;

fn move_player_projectiles(mut projectiles: Query<&mut Transform, With<PlayerProjectile>>) {
    for mut projectile in projectiles.iter_mut() {
        projectile.translation.y += PLAYER_PROJECTILE_SPEED;
    }
}

#[derive(Component)]
pub struct EnemyProjectile;

fn move_enemy_projectiles(mut projectiles: Query<&mut Transform, With<EnemyProjectile>>) {
    for mut projectile in projectiles.iter_mut() {
        projectile.translation.y -= ENEMY_PROJECTILE_SPEED;
    }
}
