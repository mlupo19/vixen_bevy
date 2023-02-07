use vixen_core::{
    game::BlockType,
    loader::{register_block, Block},
};

pub mod biomes;
mod plugin;
pub mod structures;

pub use plugin::*;

lazy_static::lazy_static! {
    static ref AIR: u16 = register_block(StandardBlocks::Air);
    static ref STONE: u16 = register_block(StandardBlocks::Stone);
    static ref GRASS: u16 = register_block(StandardBlocks::Grass);
    static ref DIRT: u16 = register_block(StandardBlocks::Dirt);
    static ref COBBLESTONE: u16 = register_block(StandardBlocks::Cobblestone);
    static ref OAK_PLANK: u16 = register_block(StandardBlocks::OakPlank);
    static ref OAK_LOG: u16 = register_block(StandardBlocks::OakLog);
    static ref OAK_LEAVES: u16 = register_block(StandardBlocks::OakLeaves);
    static ref MUSHROOM_STEM: u16 = register_block(StandardBlocks::MushroomStem);
    static ref BROWN_MUSHROOM: u16 = register_block(StandardBlocks::BrownMushroom);
    static ref RED_MUSHROOM: u16 = register_block(StandardBlocks::RedMushroom);
    static ref GOLD_ORE: u16 = register_block(StandardBlocks::GoldOre);
    static ref IRON_ORE: u16 = register_block(StandardBlocks::IronOre);
    static ref COAL_ORE: u16 = register_block(StandardBlocks::CoalOre);
    static ref SAND: u16 = register_block(StandardBlocks::Sand);
    static ref GRAVEL: u16 = register_block(StandardBlocks::Gravel);
    static ref BIRCH_LEAVES: u16 = register_block(StandardBlocks::BirchLeaves);
    static ref BIRCH_PLANK: u16 = register_block(StandardBlocks::BirchPlank);
}

pub enum StandardBlocks {
    Air,
    Stone,
    Grass,
    Dirt,
    Cobblestone,
    OakPlank,
    OakLog,
    OakLeaves,
    MushroomStem,
    BrownMushroom,
    RedMushroom,
    GoldOre,
    IronOre,
    CoalOre,
    Sand,
    Gravel,
    BirchLeaves,
    BirchPlank,
}

impl BlockType for StandardBlocks {
    fn get_name(&self) -> &'static str {
        match self {
            StandardBlocks::Air => "Air",
            StandardBlocks::Stone => "Stone",
            StandardBlocks::Grass => "Grass",
            StandardBlocks::Dirt => "Dirt",
            StandardBlocks::Cobblestone => "Cobblestone",
            StandardBlocks::OakPlank => "Oak Plank",
            StandardBlocks::OakLog => "Wood",
            StandardBlocks::OakLeaves => "Leaves",
            StandardBlocks::MushroomStem => "Mushroom Stem",
            StandardBlocks::BrownMushroom => "Brown Mushroom",
            StandardBlocks::RedMushroom => "Red Mushroom",
            StandardBlocks::GoldOre => "Gold Ore",
            StandardBlocks::IronOre => "Iron Ore",
            StandardBlocks::CoalOre => "Coal Ore",
            StandardBlocks::Sand => "Sand",
            StandardBlocks::Gravel => "Gravel",
            StandardBlocks::BirchLeaves => "Birch Leaves",
            StandardBlocks::BirchPlank => "Birch Plank",
        }
    }

    fn get_durability(&self) -> f32 {
        match self {
            StandardBlocks::Air => 0.,
            StandardBlocks::Stone => 1.,
            StandardBlocks::Grass => 1.,
            StandardBlocks::Dirt => 1.,
            StandardBlocks::Cobblestone => 1.,
            StandardBlocks::OakPlank => 1.,
            StandardBlocks::OakLog => 1.,
            StandardBlocks::OakLeaves => 1.,
            StandardBlocks::MushroomStem => 1.,
            StandardBlocks::BrownMushroom => 1.,
            StandardBlocks::RedMushroom => 1.,
            StandardBlocks::GoldOre => 1.,
            StandardBlocks::IronOre => 1.,
            StandardBlocks::CoalOre => 1.,
            StandardBlocks::Sand => 1.,
            StandardBlocks::Gravel => 1.,
            StandardBlocks::BirchLeaves => 1.,
            StandardBlocks::BirchPlank => 1.,
        }
    }

    fn get_id(&self) -> u16 {
        match self {
            StandardBlocks::Air => *AIR,
            StandardBlocks::Stone => *STONE,
            StandardBlocks::Grass => *GRASS,
            StandardBlocks::Dirt => *DIRT,
            StandardBlocks::Cobblestone => *COBBLESTONE,
            StandardBlocks::OakPlank => *OAK_PLANK,
            StandardBlocks::OakLog => *OAK_LOG,
            StandardBlocks::OakLeaves => *OAK_LEAVES,
            StandardBlocks::MushroomStem => *MUSHROOM_STEM,
            StandardBlocks::BrownMushroom => *BROWN_MUSHROOM,
            StandardBlocks::RedMushroom => *RED_MUSHROOM,
            StandardBlocks::GoldOre => *GOLD_ORE,
            StandardBlocks::IronOre => *IRON_ORE,
            StandardBlocks::CoalOre => *COAL_ORE,
            StandardBlocks::Sand => *SAND,
            StandardBlocks::Gravel => *GRAVEL,
            StandardBlocks::BirchLeaves => *BIRCH_LEAVES,
            StandardBlocks::BirchPlank => *BIRCH_PLANK,
        }
    }

    fn get_code_name(&self) -> &'static str {
        match self {
            StandardBlocks::Air => "air",
            StandardBlocks::Stone => "stone",
            StandardBlocks::Grass => "grass",
            StandardBlocks::Dirt => "dirt",
            StandardBlocks::Cobblestone => "cobblestone",
            StandardBlocks::OakPlank => "oak_planks",
            StandardBlocks::OakLog => "oak_log",
            StandardBlocks::OakLeaves => "oak_leaves",
            StandardBlocks::MushroomStem => "mushroom_stem",
            StandardBlocks::BrownMushroom => "brown_mushroom",
            StandardBlocks::RedMushroom => "red_mushroom",
            StandardBlocks::GoldOre => "gold_ore",
            StandardBlocks::IronOre => "iron_ore",
            StandardBlocks::CoalOre => "coal_ore",
            StandardBlocks::Sand => "sand",
            StandardBlocks::Gravel => "gravel",
            StandardBlocks::BirchLeaves => "birch_leaves",
            StandardBlocks::BirchPlank => "birch_planks",
        }
    }
}

impl From<StandardBlocks> for Block {
    fn from(block: StandardBlocks) -> Self {
        Block { id: block.get_id() }
    }
}

fn register_blocks() {
    // Make lazy statics execute
    let _ = StandardBlocks::Air.get_id();
    let _ = StandardBlocks::Stone.get_id();
    let _ = StandardBlocks::Grass.get_id();
    let _ = StandardBlocks::Dirt.get_id();
    let _ = StandardBlocks::Cobblestone.get_id();
    let _ = StandardBlocks::OakPlank.get_id();
    let _ = StandardBlocks::OakLog.get_id();
    let _ = StandardBlocks::OakLeaves.get_id();
    let _ = StandardBlocks::MushroomStem.get_id();
    let _ = StandardBlocks::BrownMushroom.get_id();
    let _ = StandardBlocks::RedMushroom.get_id();
    let _ = StandardBlocks::GoldOre.get_id();
    let _ = StandardBlocks::IronOre.get_id();
    let _ = StandardBlocks::CoalOre.get_id();
    let _ = StandardBlocks::Sand.get_id();
    let _ = StandardBlocks::Gravel.get_id();
    let _ = StandardBlocks::BirchLeaves.get_id();
    let _ = StandardBlocks::BirchPlank.get_id();
}
