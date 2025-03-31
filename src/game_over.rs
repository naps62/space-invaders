use bevy::{prelude::*, text::FontSmoothing};

use crate::GameState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup)
            .add_systems(Update, input.run_if(in_state(GameState::GameOver)));
    }
}

#[derive(Component)]
struct GameOverScreen;

fn setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
    use JustifyContent::*;
    use Val::*;

    let font = TextFont {
        font: asset_server.load("font.ttf"),
        font_size: 14.0,
        font_smoothing: FontSmoothing::None,
    };

    cmds.spawn((
        GameOverScreen,
        Node {
            width: Percent(100.),
            height: Percent(99.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: Center,
            padding: UiRect::all(Px(5.)),
            ..default()
        },
        BackgroundColor(Color::srgba(0., 0., 0., 0.98)),
    ))
    .with_children(|parent| {
        parent.spawn((Text::new("Game Over"), font));
    });
}

fn input(
    mut cmds: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    entities: Query<Entity, (With<Transform>, Without<Camera>)>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_game_state.set(GameState::MainMenu);

        for entity in entities.iter() {
            cmds.entity(entity).despawn();
        }
    }
}
