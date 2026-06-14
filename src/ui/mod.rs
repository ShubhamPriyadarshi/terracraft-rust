use bevy::prelude::*;

pub fn spawn_hud(mut commands: Commands) {
    // Spawn a 2D camera for UI
    commands.spawn((
        Camera2d::default(),
        Camera {
            order: 1000,
            ..default()
        },
    ));
}

pub fn update_hud() {
}
