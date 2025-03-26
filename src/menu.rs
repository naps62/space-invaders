use bevy::{prelude::*, text::FontSmoothing};

use crate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup)
            .add_systems(FixedUpdate, update.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), cleanup);
    }
}

#[derive(Component)]
struct Menu;

fn setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
    let font = TextFont {
        font: asset_server.load("font.ttf"),
        font_size: 14.0,
        font_smoothing: FontSmoothing::None,
    };

    cmds.spawn((
        Menu,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    ))
    .with_children(|parent| {
        parent.spawn((Text::new("Press SPACE to start"), font.clone()));
    });
}

fn update(keyboard: Res<ButtonInput<KeyCode>>, mut state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        state.set(GameState::Playing);
    }
}

fn cleanup(mut cmds: Commands, query: Query<Entity, With<Menu>>) {
    for entity in &query {
        cmds.entity(entity).despawn_recursive();
    }
}
