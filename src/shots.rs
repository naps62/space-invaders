use std::time::Duration;

use crate::constants::*;
use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume as _},
    prelude::*,
    time::common_conditions::on_timer,
};

pub struct ShotPlugin;

impl Plugin for ShotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup).add_systems(
            FixedUpdate,
            (
                move_player_shots,
                move_enemy_shots.run_if(on_timer(Duration::from_secs_f32(3. / 60.))),
                check_collisions,
            ),
        );
    }
}

bitflags::bitflags! {
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    struct Layer: u32 {
        const PLAYER_SHOT = 0b000001;
        const ENEMY_SHOT  = 0b000010;
        const WALL        = 0b000100;
        const ENEMY       = 0b001000;
        const PLAYER      = 0b010000;
        const SHIELD      = 0b100000;
    }
}

#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct PlayerShot;

#[derive(Component, Default)]
#[component(storage = "SparseSet")]
struct EnemyShot;

#[derive(Component, Debug)]
pub struct Collider {
    layer: Layer,
    mask: Layer,
}

#[derive(Component, Debug)]
pub struct Projectile;

impl Collider {
    pub fn player_layer() -> Self {
        Self {
            layer: Layer::PLAYER,
            mask: Layer::ENEMY_SHOT,
        }
    }

    pub fn enemy_layer() -> Self {
        Self {
            layer: Layer::ENEMY,
            mask: Layer::PLAYER_SHOT,
        }
    }

    pub fn wall_layer() -> Self {
        Self {
            layer: Layer::WALL,
            mask: Layer::PLAYER_SHOT | Layer::ENEMY_SHOT,
        }
    }

    pub fn shield_layer() -> Self {
        Self {
            layer: Layer::SHIELD,
            mask: Layer::PLAYER_SHOT | Layer::ENEMY_SHOT,
        }
    }

    pub fn should_collide(a: &Collider, b: &Collider) -> bool {
        a.mask.intersects(b.layer) && b.mask.intersects(a.layer)
    }
}

#[derive(Resource)]
pub struct SpriteWithAtlas(Sprite);

fn startup(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut atlas: ResMut<Assets<TextureAtlasLayout>>,
) {
    let a1 = assets.load("sprites/shots/enemy_a.png");
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

    // load assets
    let _ = assets.load::<Image>("sprites/shots/player.png");
    let _ = assets.load::<AudioSource>("sounds/player-shot.ogg");
}

pub fn spawn_player_shots(mut cmds: Commands, assets: Res<AssetServer>, position: Vec2) {
    let projectile = assets.load("sprites/shots/player.png");
    let mut sprite = Sprite::from_image(projectile);
    sprite.custom_size = Some(Vec2::new(1., 4.));
    cmds.spawn(AudioPlayer::new(assets.load("sounds/player-shot.ogg")));
    cmds.spawn((
        sprite,
        Transform::from_xyz(position.x, position.y, 0.0),
        PlayerShot,
        Collider {
            layer: Layer::PLAYER_SHOT,
            mask: Layer::ENEMY | Layer::WALL | Layer::SHIELD,
        },
        Projectile,
    ))
    .observe(on_hit_destroy);
}

pub fn on_hit_destroy(trigger: Trigger<Hit>, mut cmds: Commands) {
    cmds.entity(trigger.entity()).despawn();
}

pub fn spawn_enemy_shots(mut cmds: Commands, sprite: Res<SpriteWithAtlas>, position: Vec2) {
    let sprite = sprite.0.clone();
    cmds.spawn((
        sprite,
        Transform::from_xyz(position.x, position.y, 0.0),
        EnemyShot,
        Collider {
            layer: Layer::ENEMY_SHOT,
            mask: Layer::PLAYER | Layer::WALL | Layer::SHIELD,
        },
        Projectile,
    ))
    .observe(on_hit_destroy);
}

fn move_player_shots(mut shots: Query<&mut Transform, With<PlayerShot>>) {
    for mut transform in shots.iter_mut() {
        transform.translation.y += PLAYER_PROJECTILE_SPEED;
    }
}

fn move_enemy_shots(mut shots: Query<(&mut Transform, &mut Sprite), With<EnemyShot>>) {
    for (mut transform, mut sprite) in shots.iter_mut() {
        transform.translation.y -= ENEMY_PROJECTILE_SPEED;
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = (atlas.index + 1) % 4;
        }
    }
}

#[derive(Event)]
pub struct Hit;

fn check_collisions(
    par_cmds: ParallelCommands,
    projectiles: Query<(Entity, &Transform, &Sprite, &Collider), With<Projectile>>,
    colliders: Query<(Entity, &Transform, &Sprite, &Collider)>,
    images: Res<Assets<Image>>,
) {
    projectiles.par_iter().for_each(
        |(projectile_entity, projectile_transform, proj_sprite, proj_collider)| {
            for (coll_entity, coll_transform, coll_sprite, coll_collider) in colliders.iter() {
                if projectile_entity == coll_entity {
                    // it's the same entity
                    continue;
                }

                if !Collider::should_collide(proj_collider, coll_collider) {
                    // these two colliders don't collide (e.g. enemy with walls)
                    continue;
                }

                if (projectile_transform.translation.y - coll_transform.translation.y).abs() > 10. {
                    // too far away from each other on the Y axis
                    continue;
                }

                if (projectile_transform.translation.x - coll_transform.translation.x).abs() > 50. {
                    // too far away from each other on the X axis
                    continue;
                }

                let box_a = Aabb2d::new(
                    projectile_transform.translation.truncate(),
                    size(proj_sprite, projectile_transform, &images) / Vec2::new(2.0, 3.0),
                );

                let box_b = Aabb2d::new(
                    coll_transform.translation.truncate(),
                    size(coll_sprite, coll_transform, &images) / Vec2::new(2.0, 3.0),
                );

                if !box_a.intersects(&box_b) {
                    // they don't intersect
                    continue;
                }

                par_cmds.command_scope(|mut cmds| {
                    cmds.trigger_targets(Hit, projectile_entity);
                    cmds.trigger_targets(Hit, coll_entity);
                })
            }
        },
    );
}

fn size(sprite: &Sprite, transform: &Transform, images: &Res<Assets<Image>>) -> Vec2 {
    let size = if let Some(custom_size) = sprite.custom_size {
        custom_size
    } else if let Some(image) = images.get(&sprite.image) {
        image.size().as_vec2()
    } else {
        Vec2::new(1.0, 1.0)
    };

    size * transform.scale.truncate()
}
