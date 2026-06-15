#![allow(dead_code)]
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
            rendering::setup_lighting,
            setup,
            initialize_world,
            player::spawn_player,
            ui::spawn_hud,
        ))
        .add_systems(Update, (
            player::player_movement,
            player::camera_control,
            world::block_interaction,
            mob::mob_spawner,
            mob::mob_ai,
            ui::update_hud,
            debug_logging,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane - large flat grass surface
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(256.0, 256.0))),
        MeshMaterial3d(standard_materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.6, 0.2),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // Red cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(standard_materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.2, 0.2),
            perceptual_roughness: 0.5,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));

    // Blue cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(standard_materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.2, 0.9),
            perceptual_roughness: 0.5,
            ..default()
        })),
        Transform::from_xyz(10.0, 1.0, 0.0),
    ));

    // Yellow cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(standard_materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.9, 0.2),
            perceptual_roughness: 0.5,
            ..default()
        })),
        Transform::from_xyz(-10.0, 1.0, 0.0),
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

pub fn debug_logging(
    mut frame_count: Local<u32>,
    mut player_spawned: Local<bool>,
    entity_count: Query<Entity>,
    player_query: Query<(), With<player::Player>>,
    camera_count: Query<(), With<Camera3d>>,
    light_count: Query<(), With<DirectionalLight>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    *frame_count += 1;

    if !*player_spawned {
        *player_spawned = true;
        println!("[DEBUG] Entities: {}, Players: {}, Cameras: {}, Lights: {}",
            entity_count.iter().count(),
            player_query.iter().count(),
            camera_count.iter().count(),
            light_count.iter().count(),
        );
    }

    if *frame_count % 30 == 0 {
        let mut keys = Vec::new();
        if keyboard.pressed(KeyCode::KeyW) { keys.push("W"); }
        if keyboard.pressed(KeyCode::KeyS) { keys.push("S"); }
        if keyboard.pressed(KeyCode::KeyA) { keys.push("A"); }
        if keyboard.pressed(KeyCode::KeyD) { keys.push("D"); }
        if keyboard.pressed(KeyCode::Space) { keys.push("SPACE"); }
        println!("[DEBUG] Frame {}: keys=[{}] time={:.1}s",
            *frame_count, keys.join(","), time.elapsed_secs());
    }
}
