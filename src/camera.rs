use bevy::{prelude::*, render::camera::ScalingMode};

pub const RESOLUTION: f32 = 16.0 / 9.0;

pub fn spawn_camera(mut commands: Commands) {
  let mut camera = OrthographicCameraBundle::new_2d();

  camera.orthographic_projection.top = 1.0;
  camera.orthographic_projection.bottom = -1.0;
  camera.orthographic_projection.right = 1.0 * RESOLUTION;
  camera.orthographic_projection.left = -1.0 * RESOLUTION;
  camera.orthographic_projection.scaling_mode = ScalingMode::None;

  commands.spawn_bundle(camera);
}