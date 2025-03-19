use std::time::Duration;

use crate::{constants::*, player::Player};
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
    #[derive(PartialEq, Eq, Clone, Copy)]
    struct Layer: u32 {
        const PLAYER_SHOT = 0b0001;
        const ENEMY_SHOT  = 0b0010;
        const WALL        = 0b0100;
        const ENEMY       = 0b1000;
        const PLAYER      = 0b1000;
    }
}

#[derive(Component, Default)]
struct PlayerShot;

#[derive(Component, Default)]
struct EnemyShot;

#[derive(Component)]
pub struct Collider {
    layer: Layer,
    mask: Layer,
}

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
    let a1 = assets.load("shots/enemy_a.png");
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

pub fn spawn_player_shots(mut cmds: Commands, assets: Res<AssetServer>, position: Vec2) {
    let projectile = assets.load("shots/player.png");
    let mut sprite = Sprite::from_image(projectile);
    sprite.custom_size = Some(Vec2::new(1., 4.));
    cmds.spawn((
        sprite,
        Transform::from_xyz(position.x, position.y, 0.0),
        PlayerShot,
        Collider {
            layer: Layer::PLAYER_SHOT,
            mask: Layer::ENEMY | Layer::WALL,
        },
    ));
}

pub fn spawn_enemy_shots(mut cmds: Commands, sprite: Res<SpriteWithAtlas>, position: Vec2) {
    let sprite = sprite.0.clone();
    cmds.spawn((
        sprite,
        Transform::from_xyz(position.x, position.y, 0.0),
        EnemyShot,
        Collider {
            layer: Layer::ENEMY_SHOT,
            mask: Layer::PLAYER | Layer::WALL,
        },
    ));
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

fn check_collisions(
    mut cmds: Commands,
    colliders: Query<(Entity, &Transform, &Sprite, &Collider)>,
    images: Res<Assets<Image>>,
) {
    for [a, b] in colliders.iter_combinations() {
        let (entity_a, transform_a, sprite_a, collider_a) = a;
        let (entity_b, transform_b, sprite_b, collider_b) = b;

        if entity_a == entity_b {
            continue;
        }

        if !Collider::should_collide(collider_a, collider_b) {
            continue;
        }
        let box_a = Aabb2d::new(
            transform_a.translation.truncate(),
            size(sprite_a, transform_a, &images) / 2.,
        );

        let box_b = Aabb2d::new(
            transform_b.translation.truncate(),
            size(sprite_b, transform_b, &images) / 2.,
        );

        if !box_a.intersects(&box_b) {
            continue;
        }
    }

    //for (entity_a, transform_a, sprite_a, maybe_img_a, collider_a) in &colliders {
    //    for (entity_b, transform_b, sprite_b, maybe_img_b, collider_b) in &colliders {
    //        if entity_a == entity_b {
    //            continue;
    //        }
    //
    //        if !Collider::should_collide(collider_a, collider_b) {
    //            continue;
    //        }
    //
    //        let box_a = Aabb2d::new(
    //            projectile_transform.translation.truncate(),
    //            sprite_a.image.size() * transform_b.scale.truncate() / 2.,
    //        );
    //
    //        let box_b = Aabb2d::new(
    //            transform_a.translation.truncate(),
    //            sprite_b.image.size() * transform_b.scale.truncate() / 2.,
    //        );

    //for (projectile_entity, projectile_transform, projectile_sprite) in &shots {
    //    let projectile_box = Aabb2d::new(
    //        projectile_transform.translation.truncate(),
    //        projectile_sprite.custom_size.unwrap() * projectile_transform.scale.truncate()
    //            / 2.,
    //    );
    //
    //    if projectile_box.intersects(&collider_box) {
    //        cmds.entity(projectile_entity).remove::<Shot>();
    //        //let entity_type = if cmds.entity(entity_a).con
    //        events.send(CollisionEvent(
    //            ShotEntity::PlayerShot(projectile_entity),
    //            ColliderEntity::collider(entity_a),
    //        ));
    //        break;
    //    }
    //}
    //    }
    //}
}

fn size(sprite: &Sprite, transform: &Transform, images: &Res<Assets<Image>>) -> Vec2 {
    let size = if let Some(custom_size) = sprite.custom_size {
        custom_size
    } else if let Some(image) = images.get(&sprite.image) {
        dbg!("her");
        image.size().as_vec2()
    } else {
        Vec2::new(1.0, 1.0)
    };

    let res = size * transform.scale.truncate();

    res
}
