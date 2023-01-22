use std::sync::Arc;

use dashmap::DashMap;

use crate::{util::{BlockCoord, ChunkCoord}, loader::UnfinishedChunkData};

pub trait Structure {
    fn get_chance(&self) -> f64;
    fn generate(&self, position: BlockCoord, in_progress: Arc<DashMap<ChunkCoord, UnfinishedChunkData>>, rng: &mut rand::rngs::StdRng);
}