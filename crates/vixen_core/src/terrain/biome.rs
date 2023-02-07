use std::sync::Arc;

use dashmap::DashMap;

use crate::{
    loader::{ChunkData, UnfinishedChunkData},
    util::ChunkCoord,
    BlockCoord,
};

pub trait Biome: Sync + Send {
    fn get_name(&self) -> &'static str;
    fn generate_structures(
        &self,
        block_coord: &BlockCoord,
        in_progress: Arc<DashMap<ChunkCoord, UnfinishedChunkData>>,
        rng: &mut rand::rngs::StdRng,
    );
    fn generate_chunk(
        &self,
        coord: ChunkCoord,
        chunk_data: &mut ChunkData,
        in_progress: Arc<DashMap<ChunkCoord, UnfinishedChunkData>>,
        noise: &Box<dyn noise::NoiseFn<f64, 3> + Send + Sync>,
        seed: u32,
    );
}
