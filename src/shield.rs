use crate::{constants::*, shots::Collider};
use bevy::prelude::*;

pub struct ShieldPlugin;

impl Plugin for ShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup(mut cmds: Commands, assets: Res<AssetServer>) {
    let image = assets.load("shield.png");
    let size = Vec2::new(22., 18.);

    for x in [-66., -22., 22., 66.] {
        cmds.spawn((
            Shield,
            Sprite {
                image: image.clone(),
                custom_size: Some(size),
                color: Color::srgb(0., 1., 0.),
                ..default()
            },
            Transform::from_xyz(ARENA_SIZE.x / 2. + x, 70., 0.),
            Collider::shield_layer(),
        ));
    }
}

#[derive(Component)]
pub struct Shield;
