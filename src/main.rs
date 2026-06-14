mod mob;
mod player;
mod rendering;
mod ui;
mod world;

use bevy::prelude::*;
use world::WorldGenSettings;

#[derive(Resource)]
struct GameConfig {
    chunk_size: UVec2,
    world_width: u32,
    world_height: u32,
    seed: u64,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            chunk_size: UVec2::new(16, 16),
            world_width: 256,
            world_height: 128,
            seed: 42,
        }
    }
}

fn main() {
    println!("Terraria 3D - Starting game...");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Terraria 3D".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GameConfig::default())
        .add_systems(Startup, (
            setup,
            initialize_world,
            player::spawn_player,
            ui::spawn_hud,
            rendering::setup_lighting,
        ))
        .add_systems(Update, (player::player_movement, player::camera_control))
        .add_systems(Update, (world::block_interaction, mob::mob_spawner))
        .add_systems(Update, (mob::mob_ai,))
        .add_systems(Update, (ui::update_hud, world::day_night_cycle))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a ground plane
    commands.spawn(PbrEntity {
        mesh: meshes.add(Rectangle::new(256.0, 256.0)),
        material: materials.add(Color::srgb(0.2, 0.5, 0.2)),
    });
    
    // Spawn a camera looking at the world
    commands.spawn((
        Camera3d::default(),
        Camera::default(),
        Transform::from_xyz(0.0, 32.0, 64.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[derive(Component)]
struct PbrEntity {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

fn initialize_world(config: Res<GameConfig>) {
    println!("Generating world with seed {}", config.seed);
    let settings = WorldGenSettings {
        seed: config.seed,
        chunk_size: config.chunk_size,
        world_width: config.world_width,
        world_height: config.world_height,
    };
    let _chunks = world::generate_world_chunks(&settings);
}
