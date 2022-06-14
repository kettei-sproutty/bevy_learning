use bevy::{prelude::*, window::*};
mod player;
mod camera;

fn main() {
  let window_descriptor = WindowDescriptor {
    title: String::from("Bevy Learning"),
    present_mode: PresentMode::Fifo,
    resizable: false,
    width: 800.0,
    height: 600.0,
    ..Default::default()
  };

  let clear_color = ClearColor(Color::VIOLET);

  App::new()
    .insert_resource(clear_color)
    .insert_resource(window_descriptor)
    .add_plugin(player::PlayerPlugin)
    .add_startup_system(camera::spawn_camera)
    .add_plugins(DefaultPlugins)
    .run()
}
