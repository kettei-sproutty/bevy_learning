use bevy::{prelude::*, sprite::Rect};

#[derive(Clone)]
pub struct PlaceHolderGraphics {
  pub texture_atlas: Handle<TextureAtlas>,
  pub player_index: usize
}

pub fn load_graphics(
  mut commands: Commands,
  assets: Res<AssetServer>,
  mut texture_assets: ResMut<Assets<TextureAtlas>>
) {
  let image_handler = assets.load("character/char_blue.png");
  let mut atlas = TextureAtlas::new_empty(image_handler, Vec2::splat(256.0));

  let player_index = atlas.add_texture(Rect {
    min: Vec2::splat(0.0),
    max: Vec2::splat(32.0)
  });

  let texture_atlas = texture_assets.add(atlas);

  commands.insert_resource(PlaceHolderGraphics { texture_atlas, player_index });
}