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
    MushroomStem,
    BrownMushroom,
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
            StandardBlocks::MushroomStem => "Mushroom Stem",
            StandardBlocks::BrownMushroom => "Brown Mushroom",
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
            StandardBlocks::MushroomStem => 8,
            StandardBlocks::BrownMushroom => 9,
            StandardBlocks::GoldOre => 10,
            StandardBlocks::IronOre => 11,
            StandardBlocks::CoalOre => 12,
            StandardBlocks::Sand => 13,
            StandardBlocks::Gravel => 14,
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
            StandardBlocks::MushroomStem => 1.,
            StandardBlocks::BrownMushroom => 1.,
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