use bevy::prelude::*;

mod player;

// Define player sprite
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const SPRITE_SCALE: f32 = 0.5;

// Define Resource
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Bevy".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(setup_system)
        .run();
    println!("Hello, world!");
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // Add window size to resource
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    // Game textures
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
    };
    commands.insert_resource(game_textures);

    window.set_position(IVec2::new(2780, 0));
}
