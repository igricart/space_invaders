use crate::GameTextures;
use bevy::prelude::*;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, enemy_spawn_system);
    }
}

fn enemy_spawn_system(mut commands: Commands, game_textures: Res<GameTextures>) {
    // Load enemy sprite
    commands.spawn_bundle(SpriteBundle {
        texture: game_textures.enemy.clone(),
        ..Default::default()
    });
}
