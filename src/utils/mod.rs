//! Utility functions and helpers
//! Common operations used across the game

use bevy::prelude::*;

/// Logger utility
pub struct Logger;

impl Logger {
    pub fn info(msg: &str) {
        info!("{}", msg);
    }
    
    pub fn warn(msg: &str) {
        warn!("{}", msg);
    }
    
    pub fn error(msg: &str) {
        error!("{}", msg);
    }
}

/// Coordinate conversion utilities
pub struct CoordUtils;

impl CoordUtils {
    /// Convert world coordinates to chunk coordinates
    pub fn world_to_chunk(world_coord: i32, chunk_size: i32) -> i32 {
        world_coord / chunk_size
    }
    
    /// Convert world coordinates to local chunk coordinates
    pub fn world_to_local(world_coord: i32, chunk_size: i32) -> i32 {
        world_coord % chunk_size
    }
    
    /// Check if coordinates are in bounds
    pub fn is_in_bounds(x: i32, y: i32, z: i32, bounds: (i32, i32, i32)) -> bool {
        x >= 0 && x < bounds.0 && y >= 0 && y < bounds.1 && z >= 0 && z < bounds.2
    }
}

/// Random number generator utilities
pub struct RandUtils;

impl RandUtils {
    /// Generate a random float in range
    pub fn float_range(min: f32, max: f32) -> f32 {
        rand::random::<f32>().mul_add(max - min, min)
    }
    
    /// Generate a random integer in range
    pub fn int_range(min: i32, max: i32) -> i32 {
        rand::random::<i32>().rem_euclid(max - min) + min
    }
    
    /// Weighted random selection
    pub fn weighted_choice(choices: &[(f32, u32)]) -> u32 {
        let total_weight: f32 = choices.iter().map(|(w, _)| w).sum();
        let mut random = rand::random::<f32>() * total_weight;
        
        for (_, value) in choices {
            random -= value as f32;
            if random <= 0.0 {
                return *value;
            }
        }
        
        choices.last().unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_coord_conversion() {
        assert_eq!(CoordUtils::world_to_chunk(16, 16), 1);
        assert_eq!(CoordUtils::world_to_local(20, 16), 4);
    }
    
    #[test]
    fn test_bounds_check() {
        assert!(CoordUtils::is_in_bounds(5, 5, 5, (16, 16, 16)));
        assert!(!CoordUtils::is_in_bounds(20, 5, 5, (16, 16, 16)));
    }
}
