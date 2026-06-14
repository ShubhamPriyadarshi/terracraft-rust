pub mod block;
use bevy::prelude::*;
pub mod chunk;
pub mod generation;

pub use chunk::generate_world_chunks;
pub use generation::WorldGenSettings;

/// Handle block placement/removal via mouse
pub fn block_interaction(
    mouse: Res<ButtonInput<MouseButton>>,
    _player_query: Query<&Transform, With<super::player::Player>>,
) {
    if mouse.just_pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Right) {
        // TODO: Raycast to find block at cursor
    }
}

/// Day/night cycle that adjusts the directional light
pub fn day_night_cycle(
    time: Res<Time>,
    mut light_query: Query<&mut DirectionalLight>,
    mut ambient: ResMut<AmbientLight>,
) {
    let cycle = (time.elapsed_secs() * 0.05).sin();
    for mut light in light_query.iter_mut() {
        light.illuminance = (400.0 + cycle * 400.0) as f32;
    }
    ambient.brightness = (100.0 + cycle * 100.0) as f32;
}
