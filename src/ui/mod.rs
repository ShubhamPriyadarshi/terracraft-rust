use bevy::prelude::*;

pub fn spawn_hud(mut commands: Commands) {
    println!("[UI] Spawning 2D camera for HUD...");
    // Spawn a 2D camera for UI
    commands.spawn((
        Camera2d::default(),
        Camera {
            order: 1000,
            ..default()
        },
    ));
    println!("[UI] 2D camera spawned successfully");
}

pub fn update_hud() {
}
