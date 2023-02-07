mod chunk;
mod plugin;
mod registry;
mod scanner;
mod texture;
mod worldgen;

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
    utils::HashSet,
};

pub use chunk::*;
pub use plugin::*;
pub use registry::*;
pub use scanner::ChunkScanner;
pub use worldgen::ChunkMap;
pub use worldgen::UnfinishedChunkData;
pub use worldgen::Worldgen;

use crate::util::ChunkCoord;

#[derive(Component)]
pub struct ChunkBuildTask(pub Task<(ChunkCoord, Chunk)>);

#[derive(Component)]
pub struct MeshBuildTask(pub Task<MeshDataWithCoord>);

#[derive(Component)]
struct NeedsMeshBuild(pub HashSet<ChunkCoord>);

#[derive(Component)]
struct NeedsChunkBuild(pub HashSet<ChunkCoord>);

type MeshData = (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<u32>);
type MeshDataWithCoord = (ChunkCoord, MeshData);
