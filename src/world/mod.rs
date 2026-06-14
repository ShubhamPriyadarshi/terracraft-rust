pub mod chunk;
pub mod generation;
pub mod block;

pub use chunk::WorldChunk;
pub use generation::{generate_world_chunks, WorldGenSettings};
pub use block::{BlockType, get_block_type};

use bevy::prelude::*;

pub fn block_interaction(
    _mouse_input: Res<ButtonInput<MouseButton>>,
) {
}

pub fn day_night_cycle(
    time: Res<Time>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    let cycle_duration = 600.0;
    let elapsed = time.elapsed_secs() % cycle_duration;
    let cycle_progress = elapsed as f32 / cycle_duration;
    
    let sun_angle = cycle_progress * std::f32::consts::PI * 2.0;
    let brightness = (sun_angle.sin() * 0.5 + 0.5).max(0.0);
    
    ambient_light.brightness = brightness * 1000.0;
    
    let r = 0.1 + brightness * 0.5;
    let g = 0.1 + brightness * 0.6;
    let b = 0.2 + brightness * 0.7;
    ambient_light.color = Color::srgb(r, g, b);
}
