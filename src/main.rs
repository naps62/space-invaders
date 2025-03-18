mod constants;
mod enemy;
mod player;
mod shots;
mod wall;

use bevy::{prelude::*, render::camera::ScalingMode};
use constants::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use shots::ProjectilePlugin;
use wall::{Wall, WallLocation};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Space Invaders".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(ProjectilePlugin)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut cmds: Commands) {
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
}
