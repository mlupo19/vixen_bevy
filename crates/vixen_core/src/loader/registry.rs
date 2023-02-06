use std::sync::{atomic::AtomicU16, Arc};

use dashmap::{DashMap, mapref::one::Ref};

use crate::{terrain::Biome, game::BlockType};


static BIOME_COUNT: AtomicU16 = AtomicU16::new(0);
static BLOCK_COUNT: AtomicU16 = AtomicU16::new(0);

lazy_static::lazy_static! {

pub static ref BIOMES: Arc<DashMap<u16, Box<dyn Biome>>> = Arc::new(DashMap::new());
pub static ref BLOCKS: Arc<DashMap<u16, Box<dyn BlockType>>> = Arc::new(DashMap::new());
pub static ref BLOCK_IDS: Arc<DashMap<String, u16>> = Arc::new(DashMap::new());

}

pub fn register_biome(biome: impl Biome + 'static) -> u16 {
    let id = BIOME_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    BIOMES.insert(id, Box::new(biome));
    id
}

pub fn get_biome(id: u16) -> Option<Ref<'static, u16, Box<dyn Biome>>> {
    BIOMES.get(&id)
}

pub fn register_block(block: impl BlockType + 'static) -> u16 {
    let id = BLOCK_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    BLOCK_IDS.insert(block.get_code_name().to_string(), id);
    BLOCKS.insert(id, Box::new(block));
    id
}

pub fn get_block(id: u16) -> Option<Ref<'static, u16, Box<dyn BlockType>>> {
    BLOCKS.get(&id)
}

pub fn get_block_id(name: &str) -> Option<u16> {
    BLOCK_IDS.get(name).map(|r| *r)
}