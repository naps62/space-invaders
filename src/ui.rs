use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    })
    .insert(PickingBehavior::IGNORE)
    .with_children(|parent| {
        //parent.spawn((
        //    Node {
        //        width: Val::Px(10.),
        //        height: Val::Px(20.),
        //        border: UiRect::all(Val::Px(2.)),
        //        ..default()
        //    },
        //    BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
        //));
    });
}
