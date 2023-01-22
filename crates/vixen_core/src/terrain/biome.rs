use std::sync::Arc;

use dashmap::DashMap;

use crate::{loader::UnfinishedChunkData, util::ChunkCoord, BlockCoord};

pub trait Biome: Sync + Send {
    fn get_name(&self) -> &'static str;
    fn generate_structures(&self, block_coord: &BlockCoord, in_progress: Arc<DashMap<ChunkCoord, UnfinishedChunkData>>, rng: &mut rand::rngs::StdRng);
}