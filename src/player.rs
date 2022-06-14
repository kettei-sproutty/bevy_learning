use bevy::{prelude::*, sprite::Rect};

#[derive(Component)]
pub struct Player {
  pub speed: f32,
  pub jump: f32,
  pub is_jumping: bool
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) -> () {
    app
      .add_startup_system_to_stage(StartupStage::PreStartup, create_player_graphics)
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
  mut player_query: Query<(&mut Transform, &mut Player)>
) {
  let (mut player_transform, mut player) = player_query.single_mut();

  if keyboard.pressed(KeyCode::A) {
    player_transform.translation.x -= player.speed * time.delta_seconds();
    player_transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
  } else if keyboard.pressed(KeyCode::D) {
    player_transform.translation.x += player.speed * time.delta_seconds();
    player_transform.rotation = Quat::default();
  } else if keyboard.pressed(KeyCode::Space) {
    player_transform.translation.y += player.speed * player.jump * time.delta_seconds();
  } else if keyboard.pressed(KeyCode::LShift) {
    if player.speed == 1.0 {
      player.speed = 2.0;
    } else {
      player.speed = 1.0;
    }
  }
}

pub fn spawn_player(
  mut commands: Commands,
  graphics: Res<PlayerGraphics>
) {
  let mut sprite = TextureAtlasSprite::new(graphics.player_index);

  sprite.custom_size = Some(Vec2::splat(0.5));

  let player = Player {
    speed: 1.0, 
    jump: 1.0,
    is_jumping: false
  };

  commands
    .spawn_bundle(SpriteSheetBundle {
      sprite,
      texture_atlas: graphics.texture_atlas.clone(),
      ..Default::default()
    }).insert(player);

    return ()
}


#[derive(Clone)]
pub struct PlayerGraphics {
  pub texture_atlas: Handle<TextureAtlas>,
  pub player_index: usize
}

pub fn create_player_graphics(
  mut commands: Commands,
  assets: Res<AssetServer>,
  mut texture_assets: ResMut<Assets<TextureAtlas>>
) {
  let image_handler = assets.load("character/char_blue.png");
  let mut atlas = TextureAtlas::new_empty(image_handler, Vec2::splat(256.0));

  let player_index = atlas.add_texture(Rect {
    min: Vec2::splat(0.0),
    max: Vec2::splat(64.0)
  });

  let texture_atlas = texture_assets.add(atlas);

  commands.insert_resource(PlayerGraphics { texture_atlas, player_index });
}