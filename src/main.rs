use bevy::{prelude::*, window::*};
use bevy_inspector_egui::WorldInspectorPlugin;
mod player;
mod camera;
mod menu;

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
    .add_plugins(DefaultPlugins)
    .add_plugin(camera::CameraPlugin)
    .add_plugin(player::PlayerPlugin)
    .add_plugin(menu::MenuPlugin)
    .add_plugin(WorldInspectorPlugin::new())
    .run()
}
