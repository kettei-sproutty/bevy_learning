use bevy::prelude::*;
use crate::graphics::PlaceHolderGraphics;

#[derive(Component)]
pub struct PlayerPlugin {
  pub speed: f32
}

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) -> () {
    app
      .add_startup_system(spawn_player)
      .add_system(player_movement);
  }

  fn name(&self) -> &str {
      return "Player Plugin"
  }
}

pub fn player_movement(
  keyboard: Res<Input<KeyCode>>,
  time: Res<Time>,
  mut player_query: Query<(&mut Transform, &PlayerPlugin)>
) {
  let (mut player_transform, player) = player_query.single_mut();

  if keyboard.pressed(KeyCode::A) {
    player_transform.translation.x -= player.speed * time.delta_seconds();
  } else if keyboard.pressed(KeyCode::S) {
    player_transform.translation.y -= player.speed * time.delta_seconds();
  } else if keyboard.pressed(KeyCode::D) {
    player_transform.translation.x += player.speed * time.delta_seconds();
  } else if keyboard.pressed(KeyCode::W) {
    player_transform.translation.y += player.speed * time.delta_seconds();
  }
}

pub fn spawn_player(
  mut commands: Commands,
  graphics: Res<PlaceHolderGraphics>
) {
  let mut sprite = TextureAtlasSprite::new(graphics.player_index);

  sprite.custom_size = Some(Vec2::splat(0.4));

  commands
    .spawn_bundle(SpriteSheetBundle {
      sprite,
      texture_atlas: graphics.texture_atlas.clone(),
      ..Default::default()
    }).insert(PlayerPlugin { speed: 1.0 });

    return ()
}