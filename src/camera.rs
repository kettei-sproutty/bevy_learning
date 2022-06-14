use bevy::{prelude::*, render::camera::ScalingMode};
use crate::player::Player;

pub const RESOLUTION: f32 = 16.0 / 9.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(spawn_camera)
      .add_system(follow_camera);
  }
}


pub fn spawn_camera(mut commands: Commands) {
  let mut camera = OrthographicCameraBundle::new_2d();

  camera.orthographic_projection.top = 1.0;
  camera.orthographic_projection.bottom = -1.0;
  camera.orthographic_projection.right = 1.0 * RESOLUTION;
  camera.orthographic_projection.left = -1.0 * RESOLUTION;
  camera.orthographic_projection.scaling_mode = ScalingMode::None;

  commands.spawn_bundle(camera);
}

pub fn follow_camera(
  player_query: Query<&Transform, With<Player>>,
  mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>
) {
  let player_transform = player_query.single().translation;
  let mut camera_query = camera_query.single_mut();

  camera_query.translation.x = player_transform.x;
}