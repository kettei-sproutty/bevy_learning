use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
  fn build(&self, app: &mut App) {
      app
        .add_startup_system(background_music);
  }
}

fn background_music(
  asset_server: Res<AssetServer>,
  audio: Res<Audio>
) {
  let music = asset_server.load("audio/background_music.ogg");
  audio.play(music);
}