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
