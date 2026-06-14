//! Audio system for sound effects and music
//! Handles block break/place sounds, ambient sounds, and music

use bevy::prelude::*;

/// Audio resource for managing sound effects
#[derive(Resource)]
pub struct AudioResources {
    pub block_break: Option<Handle<AudioSource>>,
    pub block_place: Option<Handle<AudioSource>>,
    pub ambient: Option<Handle<AudioSource>>,
}

impl Default for AudioResources {
    fn default() -> Self {
        Self {
            block_break: None,
            block_place: None,
            ambient: None,
        }
    }
}

/// Play block break sound
pub fn play_block_break_sound(
    audio: Res<Audio>,
    audio_resources: Res<AudioResources>,
) {
    if let Some(handle) = &audio_resources.block_break {
        audio.play(handle.clone());
    }
}

/// Play block place sound
pub fn play_block_place_sound(
    audio: Res<Audio>,
    audio_resources: Res<AudioResources>,
) {
    if let Some(handle) = &audio_resources.block_place {
        audio.play(handle.clone());
    }
}

/// Setup audio system
pub fn setup_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load sound effects
    commands.insert_resource(AudioResources {
        block_break: Some(asset_server.load("sounds/block_break.ogg")),
        block_place: Some(asset_server.load("sounds/block_place.ogg")),
        ambient: Some(asset_server.load("sounds/ambient.ogg")),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audio_resources() {
        let resources = AudioResources::default();
        assert!(resources.block_break.is_none());
    }
}
