//! Block type definitions and registry
//! Defines all block types in the game with their properties

use bevy::prelude::*;

/// All possible block types in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockType {
    Air,
    Dirt,
    Grass,
    Stone,
    Wood,
    Leaves,
    Sand,
    Water,
    CoalOre,
    IronOre,
    GoldOre,
    DiamondOre,
    Bedrock,
    Planks,
    Cobblestone,
    Brick,
}

impl BlockType {
    /// Get the display name of the block
    pub fn name(&self) -> &'static str {
        match self {
            BlockType::Air => "Air",
            BlockType::Dirt => "Dirt",
            BlockType::Grass => "Grass",
            BlockType::Stone => "Stone",
            BlockType::Wood => "Wood",
            BlockType::Leaves => "Leaves",
            BlockType::Sand => "Sand",
            BlockType::Water => "Water",
            BlockType::CoalOre => "Coal Ore",
            BlockType::IronOre => "Iron Ore",
            BlockType::GoldOre => "Gold Ore",
            BlockType::DiamondOre => "Diamond Ore",
            BlockType::Bedrock => "Bedrock",
            BlockType::Planks => "Planks",
            BlockType::Cobblestone => "Cobblestone",
            BlockType::Brick => "Brick",
        }
    }
    
    /// Get the color for rendering
    pub fn color(&self) -> Color {
        match self {
            BlockType::Air => Color::NONE,
            BlockType::Dirt => Color::srgb(0.46, 0.31, 0.17),
            BlockType::Grass => Color::srgb(0.27, 0.51, 0.15),
            BlockType::Stone => Color::srgb(0.50, 0.50, 0.50),
            BlockType::Wood => Color::srgb(0.42, 0.31, 0.17),
            BlockType::Leaves => Color::srgb(0.15, 0.45, 0.10),
            BlockType::Sand => Color::srgb(0.85, 0.78, 0.58),
            BlockType::Water => Color::rgba(0.0, 0.3, 0.7, 0.6),
            BlockType::CoalOre => Color::srgb(0.20, 0.20, 0.20),
            BlockType::IronOre => Color::srgb(0.70, 0.65, 0.60),
            BlockType::GoldOre => Color::srgb(0.90, 0.75, 0.20),
            BlockType::DiamondOre => Color::srgb(0.20, 0.75, 0.85),
            BlockType::Bedrock => Color::srgb(0.15, 0.15, 0.15),
            BlockType::Planks => Color::srgb(0.60, 0.45, 0.25),
            BlockType::Cobblestone => Color::srgb(0.45, 0.45, 0.45),
            BlockType::Brick => Color::srgb(0.60, 0.25, 0.18),
        }
    }
    
    /// Get the hardness (mining time in ticks)
    pub fn hardness(&self) -> u32 {
        match self {
            BlockType::Air => 0,
            BlockType::Dirt => 10,
            BlockType::Grass => 10,
            BlockType::Stone => 50,
            BlockType::Wood => 30,
            BlockType::Leaves => 5,
            BlockType::Sand => 8,
            BlockType::Water => 0,
            BlockType::CoalOre => 60,
            BlockType::IronOre => 80,
            BlockType::GoldOre => 100,
            BlockType::DiamondOre => 150,
            BlockType::Bedrock => u32::MAX,
            BlockType::Planks => 20,
            BlockType::Cobblestone => 50,
            BlockType::Brick => 60,
        }
    }
}

/// Block registry for managing block types
pub struct BlockRegistry;

impl BlockRegistry {
    /// Get all non-air block types
    pub fn all_blocks() -> Vec<BlockType> {
        vec![
            BlockType::Dirt,
            BlockType::Grass,
            BlockType::Stone,
            BlockType::Wood,
            BlockType::Leaves,
            BlockType::Sand,
            BlockType::CoalOre,
            BlockType::IronOre,
            BlockType::GoldOre,
            BlockType::DiamondOre,
            BlockType::Planks,
            BlockType::Cobblestone,
            BlockType::Brick,
        ]
    }
}

/// Convert integer ID to block type
pub fn get_block_type(id: u32) -> BlockType {
    match id {
        0 => BlockType::Air,
        1 => BlockType::Dirt,
        2 => BlockType::Grass,
        3 => BlockType::Stone,
        4 => BlockType::Wood,
        5 => BlockType::Leaves,
        6 => BlockType::Sand,
        7 => BlockType::Water,
        8 => BlockType::CoalOre,
        9 => BlockType::IronOre,
        10 => BlockType::GoldOre,
        11 => BlockType::DiamondOre,
        12 => BlockType::Bedrock,
        13 => BlockType::Planks,
        14 => BlockType::Cobblestone,
        15 => BlockType::Brick,
        _ => BlockType::Air,
    }
}

/// Convert block type to integer ID
pub fn block_type_to_id(block: BlockType) -> u32 {
    match block {
        BlockType::Air => 0,
        BlockType::Dirt => 1,
        BlockType::Grass => 2,
        BlockType::Stone => 3,
        BlockType::Wood => 4,
        BlockType::Leaves => 5,
        BlockType::Sand => 6,
        BlockType::Water => 7,
        BlockType::CoalOre => 8,
        BlockType::IronOre => 9,
        BlockType::GoldOre => 10,
        BlockType::DiamondOre => 11,
        BlockType::Bedrock => 12,
        BlockType::Planks => 13,
        BlockType::Cobblestone => 14,
        BlockType::Brick => 15,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_block_type_conversion() {
        let block = BlockType::Stone;
        let id = block_type_to_id(block);
        assert_eq!(get_block_type(id), block);
    }
    
    #[test]
    fn test_block_colors() {
        assert_ne!(BlockType::Air.color(), BlockType::Stone.color());
        assert_ne!(BlockType::Dirt.color(), BlockType::Grass.color());
    }
}
