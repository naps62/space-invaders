use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup(mut cmds: Commands, assets: Res<AssetServer>) {
    let music = assets.load::<AudioSource>("sounds/music.ogg");
    cmds.spawn(AudioPlayer(music));
}
