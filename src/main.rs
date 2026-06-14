mod mob;
mod player;
mod rendering;
mod ui;
mod world;

use bevy::prelude::*;
use bevy::pbr::MeshMaterial3d;
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
        .add_systems(Update, (
            player::player_movement,
            player::camera_control,
            world::block_interaction,
            mob::mob_spawner,
            mob::mob_ai,
            ui::update_hud,
            world::day_night_cycle,
            debug_logging,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a ground plane - rotated to lie flat on the ground (XZ plane)
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(256.0, 256.0))),
        MeshMaterial3d(standard_materials.add(Color::srgb(0.2, 0.5, 0.2))),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    
    // Spawn a red cube in the center so you have something to look at
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(standard_materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));
    
    // Spawn a blue cube to the right
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(standard_materials.add(Color::srgb(0.2, 0.2, 0.8))),
        Transform::from_xyz(10.0, 1.0, 0.0),
    ));
    
    // Spawn a yellow cube to the left
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(standard_materials.add(Color::srgb(0.9, 0.9, 0.2))),
        Transform::from_xyz(-10.0, 1.0, 0.0),
    ));
    
    // Spawn a camera looking at the world
    commands.spawn((
        Camera3d::default(),
        Camera::default(),
        Transform::from_xyz(0.0, 8.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn initialize_world(config: Res<GameConfig>) {
    println!("[WORLD] Generating world with seed {}", config.seed);
    let settings = WorldGenSettings {
        seed: config.seed,
        chunk_size: config.chunk_size,
        world_width: config.world_width,
        world_height: config.world_height,
    };
    let chunks = world::generate_world_chunks(&settings);
    println!("[WORLD] Generated {} chunks successfully", chunks.len());
}

/// Debug system to log entity counts every 60 frames
pub fn debug_logging(
    mut frame_count: Local<u32>,
    entity_count: Query<Entity>,
    player_count: Query<(), With<player::Player>>,
    camera_count: Query<(), With<Camera3d>>,
    light_count: Query<(), With<DirectionalLight>>,
) {
    *frame_count += 1;
    if *frame_count % 60 == 0 {
        println!("[DEBUG] Frame {}: entities={}, players={}, cameras={}, lights={}",
            *frame_count,
            entity_count.iter().count(),
            player_count.iter().count(),
            camera_count.iter().count(),
            light_count.iter().count(),
        );
    }
}
