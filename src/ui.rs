use crate::{constants::*, score::Score};
use bevy::{prelude::*, text::FontSmoothing, window::WindowResized};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                update_scale,
                update_score.run_if(resource_changed::<Score>),
                update_lives,
            ),
        );
    }
}

#[derive(Component)]
struct ScoreIndicator;

#[derive(Component)]
struct LivesIndicator;

#[derive(Component)]
struct LivesImagesIndicator;

fn setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
    use FlexDirection::*;
    use JustifyContent::*;
    use Val::*;

    let font = TextFont {
        font: asset_server.load("font.ttf"),
        font_size: 10.0,
        font_smoothing: FontSmoothing::None,
    };
    let color: TextColor = Color::srgb(0.7, 0.7, 0.7).into();

    cmds.spawn(Node {
        width: Percent(100.),
        height: Percent(100.),
        flex_direction: Column,
        align_items: AlignItems::Stretch,
        justify_content: SpaceBetween,
        padding: UiRect::all(Px(5.)),
        ..default()
    })
    .with_children(|parent| {
        // top text
        parent
            .spawn({
                Node {
                    width: Percent(100.),
                    flex_direction: Column,
                    ..default()
                }
            })
            .with_children(|parent| {
                parent
                    .spawn({
                        Node {
                            width: Percent(100.),
                            height: Px(50.),
                            flex_direction: Row,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        }
                    })
                    .with_children(|parent| {
                        parent
                            .spawn({
                                Node {
                                    flex_direction: Column,
                                    ..default()
                                }
                            })
                            .with_children(|parent| {
                                parent.spawn((Text::new("Score <1>"), font.clone(), color));
                                parent.spawn((ScoreIndicator, Text::new(""), font.clone(), color));
                            });

                        parent
                            .spawn({
                                Node {
                                    flex_direction: Column,
                                    ..default()
                                }
                            })
                            .with_children(|parent| {
                                parent.spawn((Text::new("Hi-Score"), font.clone(), color));
                                parent.spawn((Text::new(" 0000"), font.clone(), color));
                            });

                        parent
                            .spawn({
                                Node {
                                    flex_direction: Column,
                                    ..default()
                                }
                            })
                            .with_children(|parent| {
                                parent.spawn((Text::new("Score <2>"), font.clone(), color));
                                parent.spawn((Text::new(""), font.clone(), color));
                            });
                        // hi-score
                    });
            });

        parent
            .spawn(Node {
                flex_direction: FlexDirection::Column,
                ..default()
            })
            .with_children(|parent| {
                // bottom text
                parent
                    .spawn(Node {
                        width: Val::Percent(100.),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn(Node {
                                height: Percent(100.),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn((LivesIndicator, (Text::new("3"), font.clone(), color)));
                                parent.spawn((
                                    Node {
                                        height: Percent(100.),
                                        ..default()
                                    },
                                    LivesImagesIndicator,
                                ));
                            });
                        parent.spawn((Text::new("Credit 00"), font.clone(), color));
                    });
            });
    });
}

fn update_scale(
    mut e: EventReader<WindowResized>,
    mut ui_scale: ResMut<UiScale>,
    window: Single<&Window>,
) {
    let window = window.into_inner();

    // we don't actually care about the events, just clear them and process scale once
    if !e.is_empty() {
        e.clear();
        let scale = window.size() / ARENA_SIZE;
        ui_scale.0 = scale.x.min(scale.y);
    }
}

fn update_score(score: Res<Score>, indicator: Single<&mut Text, With<ScoreIndicator>>) {
    let mut indicator = indicator.into_inner();
    *indicator = Text::new(format!(" {}", score.0));
}

fn update_lives() {}
