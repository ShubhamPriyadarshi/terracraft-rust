use bevy::prelude::*;

pub struct WorldChunk {
    pub x: i32,
    pub z: i32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub blocks: Vec<u32>,
}

impl WorldChunk {
    pub fn new(x: i32, z: i32, width: u32, height: u32, depth: u32) -> Self {
        let total_blocks = (width * depth * height) as usize;
        Self {
            x, z, width, height, depth,
            blocks: vec![0; total_blocks],
        }
    }
    
    #[inline]
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<u32> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 || z < 0 || z >= self.depth as i32 {
            return None;
        }
        let idx = (x + self.x * self.width as i32) as usize
            + (z + self.z * self.depth as i32) as usize * self.width as usize
            + y as usize * self.width as usize * self.depth as usize;
        if idx < self.blocks.len() {
            Some(self.blocks[idx])
        } else {
            None
        }
    }
    
    #[inline]
    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block_id: u32) {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 || z < 0 || z >= self.depth as i32 {
            return;
        }
        let idx = (x + self.x * self.width as i32) as usize
            + (z + self.z * self.depth as i32) as usize * self.width as usize
            + y as usize * self.width as usize * self.depth as usize;
        if idx < self.blocks.len() {
            self.blocks[idx] = block_id;
        }
    }
}
