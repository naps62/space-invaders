use std::time::Duration;

use crate::{constants::*, enemy::Enemy, player::Player};
use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume as _},
    prelude::*,
    time::common_conditions::on_timer,
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyHit>()
            .add_event::<PlayerHit>()
            .add_systems(Startup, startup)
            .add_systems(
                FixedUpdate,
                (
                    move_player_projectiles,
                    move_enemy_projectiles.run_if(on_timer(Duration::from_secs_f32(3. / 60.))),
                    check_player_projectile_collisions,
                    check_enemy_projectile_collisions,
                    on_enemy_hit,
                    on_player_hit,
                ),
            );
    }
}

#[derive(Resource)]
pub struct SpriteWithAtlas(Sprite);

fn startup(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut atlas: ResMut<Assets<TextureAtlasLayout>>,
) {
    let a1 = assets.load("projectiles/enemy_a.png");
    let a1_atlas = atlas.add(TextureAtlasLayout::from_grid(
        UVec2::new(3, 7),
        4,
        1,
        Some(UVec2::splat(1)),
        None,
    ));
    let mut sprite = Sprite::from_atlas_image(
        a1,
        TextureAtlas {
            layout: a1_atlas,
            index: 0,
        },
    );
    sprite.custom_size = Some(Vec2::new(1., 4.));

    cmds.insert_resource(SpriteWithAtlas(sprite));
}

#[derive(Component, Default)]
pub struct Collider;

pub fn spawn_player_projectiles(mut cmds: Commands, assets: Res<AssetServer>, position: Vec2) {
    let projectile = assets.load("projectiles/player.png");
    let mut sprite = Sprite::from_image(projectile);
    sprite.custom_size = Some(Vec2::new(1., 4.));
    cmds.spawn((
        sprite,
        Transform::from_xyz(position.x, position.y, 0.0),
        PlayerProjectile,
        Collider,
    ));
}

pub fn spawn_enemy_projectiles(mut cmds: Commands, sprite: Res<SpriteWithAtlas>, position: Vec2) {
    let sprite = sprite.0.clone();
    cmds.spawn((
        sprite,
        Transform::from_xyz(position.x, position.y, 0.0),
        EnemyProjectile,
        Collider,
    ));
}

#[derive(Component)]
struct PlayerProjectile;

fn move_player_projectiles(mut projectiles: Query<&mut Transform, With<PlayerProjectile>>) {
    for mut transform in projectiles.iter_mut() {
        transform.translation.y += PLAYER_PROJECTILE_SPEED;
    }
}

#[derive(Component)]
struct EnemyProjectile;

#[derive(Event, Debug)]
pub struct EnemyHit {
    projectile: Entity,
    enemy: Entity,
}

#[derive(Event, Debug)]
pub struct PlayerHit {
    projectile: Entity,
}

fn move_enemy_projectiles(
    mut projectiles: Query<(&mut Transform, &mut Sprite), With<EnemyProjectile>>,
) {
    for (mut transform, mut sprite) in projectiles.iter_mut() {
        transform.translation.y -= ENEMY_PROJECTILE_SPEED;
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = (atlas.index + 1) % 4;
        }
    }
}

fn check_player_projectile_collisions(
    mut cmds: Commands,
    projectiles: Query<(Entity, &Transform, &Sprite), (With<PlayerProjectile>, With<Collider>)>,
    enemies: Query<(Entity, &Transform, &Sprite), With<Enemy>>,
    mut events: EventWriter<EnemyHit>,
) {
    for (enemy_entity, enemy_transform, enemy_sprite) in &enemies {
        let enemy_box = Aabb2d::new(
            enemy_transform.translation.truncate(),
            enemy_sprite.custom_size.unwrap() * enemy_transform.scale.truncate() / 2.,
        );
        for (projectile_entity, projectile_transform, projectile_sprite) in &projectiles {
            let projectile_box = Aabb2d::new(
                projectile_transform.translation.truncate(),
                projectile_sprite.custom_size.unwrap() * projectile_transform.scale.truncate() / 2.,
            );

            if projectile_box.intersects(&enemy_box) {
                cmds.entity(projectile_entity).remove::<Collider>();
                events.send(EnemyHit {
                    projectile: projectile_entity,
                    enemy: enemy_entity,
                });
                break;
            }
        }
    }
}

fn check_enemy_projectile_collisions(
    mut cmds: Commands,
    projectiles: Query<(Entity, &Transform, &Sprite), (With<EnemyProjectile>, With<Collider>)>,
    player: Single<(&Transform, &Sprite), With<Player>>,
    mut events: EventWriter<PlayerHit>,
) {
    let (player_transform, player_sprite) = player.into_inner();

    let player_box = Aabb2d::new(
        player_transform.translation.truncate(),
        player_sprite.custom_size.unwrap() * player_transform.scale.truncate() / 2.,
    );
    for (projectile_entity, projectile_transform, projectile_sprite) in &projectiles {
        let projectile_box = Aabb2d::new(
            projectile_transform.translation.truncate(),
            projectile_sprite.custom_size.unwrap() * projectile_transform.scale.truncate() / 2.,
        );

        if projectile_box.intersects(&player_box) {
            cmds.entity(projectile_entity).remove::<Collider>();
            events.send(PlayerHit {
                projectile: projectile_entity,
            });
            break;
        }
    }
}

fn on_enemy_hit(mut cmds: Commands, mut events: EventReader<EnemyHit>) {
    for ev in events.read() {
        cmds.entity(ev.enemy).despawn();
        cmds.entity(ev.projectile).despawn();
    }
}

fn on_player_hit(cmds: Commands, mut events: EventReader<PlayerHit>) {
    for ev in events.read() {
        dbg!("player hit");
    }
}
