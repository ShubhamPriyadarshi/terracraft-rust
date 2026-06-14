use super::generation::WorldGenSettings;
use super::block::BlockType;

#[derive(Debug)]
pub struct BlockChunk {
    pub blocks: Vec<BlockType>,
    #[allow(dead_code)]
    pub size: u32,
}

impl BlockChunk {
    pub fn new(size: u32) -> Self {
        Self {
            blocks: vec![BlockType::Air; (size * size * size) as usize],
            size,
        }
    }
}

pub fn generate_world_chunks(_settings: &WorldGenSettings) -> Vec<BlockChunk> {
    let mut chunks = Vec::new();
    let chunk_size = _settings.chunk_size.x;
    let num_chunks = (_settings.world_width / chunk_size) as usize;
    
    for i in 0..num_chunks {
        let mut chunk = BlockChunk::new(chunk_size);
        for z in 0..chunk_size {
            for x in 0..chunk_size {
                let global_x = (i as u32) * chunk_size + x;
                let block = if global_x < 128 {
                    BlockType::Dirt
                } else {
                    BlockType::Stone
                };
                let idx = (z * chunk_size * chunk_size + x * chunk_size) as usize;
                if idx < chunk.blocks.len() {
                    chunk.blocks[idx] = block;
                }
            }
        }
        chunks.push(chunk);
    }
    chunks
}
