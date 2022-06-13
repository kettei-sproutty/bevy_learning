use bevy::prelude::*;
use crate::graphics::PlaceHolderGraphics;

pub fn spawn_player(
  mut commands: Commands,
  graphics: Res<PlaceHolderGraphics>
) {
  let mut sprite = TextureAtlasSprite::new(graphics.player_index);

  sprite.custom_size = Some(Vec2::splat(0.4));

  commands.spawn_bundle(SpriteSheetBundle {
    sprite,
    texture_atlas: graphics.texture_atlas.clone(),
    ..Default::default()
  });
}