use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score::default())
            .add_event::<ScoreChanged>()
            .add_observer(on_points_removal);
    }
}

#[derive(Resource, Default)]
pub struct Score(pub usize);

#[derive(Component, Debug)]
pub struct Points(pub usize);

#[derive(Event, Default)]
pub struct ScoreChanged {
    pub score: usize,
}

fn on_points_removal(
    trigger: Trigger<OnRemove, Points>,
    points: Query<&Points>,
    mut score: ResMut<Score>,
) {
    let entity = trigger.entity();
    if let Ok(points) = points.get(entity) {
        score.0 += points.0;
        dbg!(score.0);
    }
}
