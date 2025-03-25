use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

use crate::{
    constants::*,
    shots::{Collider, Hit},
};
use bevy::prelude::*;

pub struct ShieldPlugin;

impl Plugin for ShieldPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LoadingState>()
            .add_systems(Startup, startup)
            .add_systems(
                Update,
                // shields need the sprite to be loaded to spawn, hence this roundabout logic.
                // mostly adapted from https://bevyengine.org/examples/assets/multi-asset-sync/
                spawn_sprites.run_if(is_sprite_loaded.and(in_state(LoadingState::Loading))),
            );
    }
}

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq, Ord, PartialOrd, States)]
pub enum LoadingState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Debug, Resource)]
struct Asset(Handle<Image>);

#[derive(Debug, Resource, Deref)]
struct AssetBarrier(Arc<AssetBarrierInner>);

#[derive(Debug, Deref)]
struct AssetBarrierGuard(Arc<AssetBarrierInner>);

#[derive(Debug, Resource)]
pub struct AssetBarrierInner {
    count: AtomicU32,
}

impl AssetBarrier {
    fn new() -> (AssetBarrier, AssetBarrierGuard) {
        let inner = Arc::new(AssetBarrierInner {
            count: AtomicU32::new(1),
        });
        (AssetBarrier(inner.clone()), AssetBarrierGuard(inner))
    }

    fn is_ready(&self) -> bool {
        self.count.load(Ordering::Acquire) == 0
    }
}

// Increment count on clone.
impl Clone for AssetBarrierGuard {
    fn clone(&self) -> Self {
        self.count.fetch_add(1, Ordering::AcqRel);
        AssetBarrierGuard(self.0.clone())
    }
}

// Decrement count on drop.
impl Drop for AssetBarrierGuard {
    fn drop(&mut self) {
        self.count.fetch_sub(1, Ordering::AcqRel);
    }
}

fn startup(mut cmds: Commands, assets: Res<AssetServer>) {
    let (barrier, guard) = AssetBarrier::new();
    cmds.insert_resource(Asset(
        assets.load_acquire("sprites/shield.png", guard.clone()),
    ));
    cmds.insert_resource(barrier);
}

fn is_sprite_loaded(barrier: Option<Res<AssetBarrier>>) -> bool {
    barrier.map(|b| b.is_ready()) == Some(true)
}

fn spawn_sprites(
    mut cmds: Commands,
    sprite: Res<Asset>,
    mut next_loading_state: ResMut<NextState<LoadingState>>,
    images: Res<Assets<Image>>,
) {
    next_loading_state.set(LoadingState::Loaded);

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
                custom_size: Some(Vec2::splat(1.)),
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
