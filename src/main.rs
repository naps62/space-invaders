mod camera;
mod constants;
mod enemy;
mod game_over;
mod hud;
mod menu;
mod player;
mod score;
mod shield;
mod shots;
mod wall;

use bevy::prelude::*;
use constants::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    GameOver,
}

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
        .init_state::<GameState>()
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_plugins(wall::WallPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(shield::ShieldPlugin)
        .add_plugins(shots::ShotPlugin)
        .add_plugins(hud::HudPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(score::ScorePlugin)
        .add_plugins(menu::MenuPlugin)
        .add_plugins(game_over::GameOverPlugin)
        .run();
}
