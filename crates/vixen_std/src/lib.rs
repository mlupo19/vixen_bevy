use vixen_core::{game::BlockType, loader::Block};

pub mod biomes;
pub mod structures;
mod plugin;

pub use plugin::*;

pub enum StandardBlocks {
    Air,
    Stone,
    Grass,
    Dirt,
    Cobblestone,
    OakPlanks,
    OakLog,
    OakLeaves,
    GoldOre,
    IronOre,
    CoalOre,
    Sand,
    Gravel,
}

impl BlockType for StandardBlocks {
    fn get_name(&self) -> &'static str {
        match self {
            StandardBlocks::Air => "Air",
            StandardBlocks::Stone => "Stone",
            StandardBlocks::Grass => "Grass",
            StandardBlocks::Dirt => "Dirt",
            StandardBlocks::Cobblestone => "Cobblestone",
            StandardBlocks::OakPlanks => "Oak Planks",
            StandardBlocks::OakLog => "Wood",
            StandardBlocks::OakLeaves => "Leaves",
            StandardBlocks::GoldOre => "Gold Ore",
            StandardBlocks::IronOre => "Iron Ore",
            StandardBlocks::CoalOre => "Coal Ore",
            StandardBlocks::Sand => "Sand",
            StandardBlocks::Gravel => "Gravel",
        }
    }

    fn get_id(&self) -> u16 {
        match self {
            StandardBlocks::Air => 0,
            StandardBlocks::Stone => 1,
            StandardBlocks::Grass => 2,
            StandardBlocks::Dirt => 3,
            StandardBlocks::Cobblestone => 4,
            StandardBlocks::OakPlanks => 5,
            StandardBlocks::OakLog => 6,
            StandardBlocks::OakLeaves => 7,
            StandardBlocks::GoldOre => 8,
            StandardBlocks::IronOre => 9,
            StandardBlocks::CoalOre => 10,
            StandardBlocks::Sand => 11,
            StandardBlocks::Gravel => 12,
        }
    }

    fn get_durability(&self) -> f32 {
        match self {
            StandardBlocks::Air => 0.,
            StandardBlocks::Stone => 1.,
            StandardBlocks::Grass => 1.,
            StandardBlocks::Dirt => 1.,
            StandardBlocks::Cobblestone => 1.,
            StandardBlocks::OakPlanks => 1.,
            StandardBlocks::OakLog => 1.,
            StandardBlocks::OakLeaves => 1.,
            StandardBlocks::GoldOre => 1.,
            StandardBlocks::IronOre => 1.,
            StandardBlocks::CoalOre => 1.,
            StandardBlocks::Sand => 1.,
            StandardBlocks::Gravel => 1.,
        }
    }
}

impl From<StandardBlocks> for Block {
    fn from(block: StandardBlocks) -> Self {
        Block {
            id: block.get_id(),
            durability: block.get_durability(),
        }
    }
}