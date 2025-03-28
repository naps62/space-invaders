use crate::{constants::*, player::Lives, score::Score, GameState};
use bevy::{prelude::*, text::FontSmoothing, ui::widget::ImageNodeSize, window::WindowResized};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                (
                    update_scale,
                    update_score.run_if(resource_changed::<Score>),
                    update_lives.run_if(resource_changed::<Lives>),
                )
                    .run_if(in_state(GameState::Playing)),
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
                                parent.spawn((
                                    ScoreIndicator,
                                    Text::new(" 0000"),
                                    font.clone(),
                                    color,
                                ));
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
                                parent
                                    .spawn((
                                        Node {
                                            height: Percent(100.),
                                            ..default()
                                        },
                                        LivesImagesIndicator,
                                    ))
                                    .with_children(|parent| {
                                        let mut image =
                                            ImageNode::new(asset_server.load("sprites/player.png"));
                                        image.color = Color::srgb(0., 1., 0.);
                                        let node = Node {
                                            width: Px(PLAYER_SIZE.x),
                                            height: Px(PLAYER_SIZE.y),
                                            margin: UiRect::new(Px(5.), Px(0.), Px(0.), Px(0.)),
                                            ..default()
                                        };
                                        parent.spawn((node.clone(), image.clone()));
                                        parent.spawn((node.clone(), image.clone()));
                                    });
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

fn update_lives(
    mut cmds: Commands,
    lives: Res<Lives>,
    indicator: Single<&mut Text, With<LivesIndicator>>,
    images_indicator: Single<(Entity, Option<&Children>), With<LivesImagesIndicator>>,
) {
    let mut indicator = indicator.into_inner();
    *indicator = Text::new(format!(" {}", lives.0));

    let (parent, children) = images_indicator.into_inner();
    if let Some(children) = children {
        if let Some(&last_child) = children.iter().last() {
            //cmds.entity(last_child).despawn();
        }
    }
}
