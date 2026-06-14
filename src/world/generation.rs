use rand::Rng;
use rayon::prelude::*;
use crate::world::WorldChunk;

pub struct WorldGenSettings {
    pub seed: u64,
    pub chunk_size: bevy::prelude::UVec2,
    pub world_width: u32,
    pub world_height: u32,
}

pub fn generate_world_chunks(settings: &WorldGenSettings) -> Vec<WorldChunk> {
    let num_chunks_x = (settings.world_width / settings.chunk_size.x) as usize;
    let num_chunks_z = (settings.world_width / settings.chunk_size.y) as usize;
    let total_chunks = num_chunks_x * num_chunks_z;
    
    println!("Generating {} chunks...", total_chunks);
    
    let chunks: Vec<WorldChunk> = (0..num_chunks_x)
        .into_par_iter()
        .flat_map(|x| {
            (0..num_chunks_z)
                .map(|z| {
                    generate_chunk(x, z, settings)
                })
                .collect::<Vec<_>>()
        })
        .collect();
    
    println!("Generated {} chunks", chunks.len());
    chunks
}

fn generate_chunk(
    chunk_x: usize,
    chunk_z: usize,
    settings: &WorldGenSettings,
) -> WorldChunk {
    let mut rng = rand::thread_rng();
    
    let chunk = WorldChunk::new(
        chunk_x as i32 * settings.chunk_size.x as i32,
        chunk_z as i32 * settings.chunk_size.y as i32,
        settings.chunk_size.x,
        settings.chunk_size.y,
        settings.chunk_size.y,
    );
    
    let mut heightmap = vec![0u32; settings.chunk_size.x as usize * settings.chunk_size.y as usize];
    
    for x in 0..settings.chunk_size.x {
        for z in 0..settings.chunk_size.y {
            let world_x = chunk.x + x as i32;
            let world_z = chunk.z + z as i32;
            
            let height = (world_x as f32 * 0.1).sin() * 5.0 + (world_z as f32 * 0.1).sin() * 5.0 + 20.0;
            let height_block = height as u32;
            heightmap[(x * settings.chunk_size.y + z) as usize] = height_block.min(settings.chunk_size.y - 5);
        }
    }
    
    let mut filled_chunk = chunk;
    
    for x in 0..settings.chunk_size.x {
        for z in 0..settings.chunk_size.y {
            let height = heightmap[(x * settings.chunk_size.y + z) as usize];
            
            for y in 0..settings.chunk_size.y {
                let block_id = if y == 0 {
                    12
                } else if y < height - 4 {
                    3
                } else if y < height {
                    1
                } else if y == height {
                    2
                } else {
                    0
                };
                
                filled_chunk.set_block(x as i32, y as i32, z as i32, block_id);
            }
        }
    }
    
    add_simple_trees(&mut filled_chunk, &mut rng, settings);
    
    filled_chunk
}

fn add_simple_trees(
    chunk: &mut WorldChunk,
    rng: &mut rand::prelude::ThreadRng,
    settings: &WorldGenSettings,
) {
    for x in 2..settings.chunk_size.x - 2 {
        for z in 2..settings.chunk_size.y - 2 {
            if rng.gen_bool(0.02) {
                let world_x = chunk.x + x as i32;
                let world_z = chunk.z + z as i32;
                
                for y in (0..settings.chunk_size.y as i32).rev() {
                    if chunk.get_block(x as i32, y, z as i32) == Some(2) {
                        let trunk_height = rng.gen_range(4..8);
                        for ty in 0..trunk_height {
                            chunk.set_block(world_x, y + ty + 1, world_z, 4);
                        }
                        break;
                    }
                }
            }
        }
    }
}
