#[derive(Debug)]
pub struct WorldGenSettings {
    pub seed: u64,
    #[allow(dead_code)]
    pub chunk_size: crate::UVec2,
    pub world_width: u32,
    #[allow(dead_code)]
    pub world_height: u32,
}

impl WorldGenSettings {
    pub fn new(chunk_size: crate::UVec2, world_width: u32) -> Self {
        Self {
            seed: 42,
            chunk_size,
            world_width,
            world_height: 128,
        }
    }
}
