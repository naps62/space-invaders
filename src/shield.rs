use crate::{
    constants::*,
    shots::{Collider, Hit},
    GameState,
};
use bevy::prelude::*;

pub struct ShieldPlugin;

#[derive(Resource, Eq, PartialEq)]
struct HasSpawnedSprites(bool);

impl Plugin for ShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), startup)
            .add_systems(
                FixedUpdate,
                spawn_sprites.run_if(
                    in_state(GameState::Playing)
                        .and(is_sprite_loaded)
                        .and(resource_exists_and_equals(HasSpawnedSprites(false))),
                ),
            );
    }
}

#[derive(Debug, Resource)]
struct Asset(Handle<Image>);

fn startup(mut cmds: Commands, assets: Res<AssetServer>) {
    cmds.insert_resource(Asset(assets.load("sprites/shield.png")));
    cmds.insert_resource(HasSpawnedSprites(false));
}

fn is_sprite_loaded(sprite: Res<Asset>, asset_server: Res<AssetServer>) -> bool {
    asset_server.load_state(&sprite.0).is_loaded()
}

fn spawn_sprites(
    mut cmds: Commands,
    sprite: Res<Asset>,
    images: Res<Assets<Image>>,
    mut has_spawned_sprites: ResMut<HasSpawnedSprites>,
) {
    has_spawned_sprites.0 = true;
    let image = images.get(&sprite.0).unwrap();
    let width = image.size().x as usize;
    let height = image.size().y as usize;

    let y_offset = 77.0;

    // spawn each shield
    for shield in [33., 79., 124., 168.].iter() {
        for y in 0..height {
            for x in 0..width {
                let pixel_index = (y * width + x) * 4; // RGBA format
                let alpha = image.data[pixel_index + 3]; // Check transparency

                if alpha > 128 {
                    // Only spawn blocks if pixel is visible
                    let block_x = shield + x as f32;
                    let block_y = y_offset - y as f32;

                    cmds.spawn((
                        ShieldBlock,
                        Sprite {
                            color: GREEN,
                            custom_size: Some(Vec2::splat(1.)),
                            ..default()
                        },
                        Transform::from_xyz(block_x, block_y, 0.0),
                        Collider::shield_layer(),
                    ))
                    .observe(on_hit);
                }
            }
        }
    }

    // spawn bottom line!()
    for x in 0..(ARENA_SIZE.x as u32) {
        cmds.spawn((
            ShieldBlock,
            Sprite {
                color: GREEN,
                custom_size: Some(Vec2::new(1., 0.5)),
                ..default()
            },
            Transform::from_xyz(x as f32, 20., 0.0),
            Collider::shield_layer(),
        ))
        .observe(on_hit);
    }
}

#[derive(Component)]
struct ShieldBlock;

fn on_hit(trigger: Trigger<Hit>, mut cmds: Commands) {
    let entity = trigger.entity();
    cmds.entity(entity).despawn();
}
