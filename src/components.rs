use bevy::prelude::Component;

// Common components
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

// Player Components
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,
}
