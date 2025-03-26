mod camera;
mod constants;
mod enemy;
mod player;
mod score;
mod shield;
mod shots;
mod ui;
mod wall;

use bevy::prelude::*;
use constants::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
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
        .add_plugins(wall::WallPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(shield::ShieldPlugin)
        .add_plugins(shots::ShotPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(score::ScorePlugin)
        .run();
}
