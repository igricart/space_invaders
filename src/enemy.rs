use crate::{GameTextures, WinSize, SPRITE_SCALE};
use bevy::prelude::*;
use rand::Rng;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, enemy_spawn_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    // Add random position to enemy
    let mut rng = rand::thread_rng();
    let w_span = win_size.w / 2.0 - 100.0;
    let h_span = win_size.h / 2.0 - 100.0;
    let x = rng.gen_range(-w_span..w_span);
    let y = rng.gen_range(-h_span..h_span);

    // Load enemy sprite
    commands.spawn_bundle(SpriteBundle {
        texture: game_textures.enemy.clone(),
        transform: Transform {
            translation: Vec3::new(x, y, 10.0),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
