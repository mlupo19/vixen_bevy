mod scanner;
mod chunk;
mod texture;
mod worldgen;
mod plugin;
mod registry;

use bevy::{prelude::*, utils::HashSet, tasks::{AsyncComputeTaskPool, Task}};

pub use scanner::ChunkScanner;
pub use worldgen::Worldgen;
pub use worldgen::ChunkMap;
pub use worldgen::UnfinishedChunkData;
pub use chunk::*;
pub use plugin::*;
pub use registry::*;

use crate::util::ChunkCoord;

#[derive(Component)]
pub struct ChunkBuildTask(pub Task<(ChunkCoord, Chunk)>);

#[derive(Component)]
pub struct MeshBuildTask(pub Task<MeshDataWithCoord>);

#[derive(Component)]
struct NeedsMeshBuild(pub HashSet<ChunkCoord>);

#[derive(Component)]
struct NeedsChunkBuild(pub HashSet<ChunkCoord>);

type MeshData = (Vec<[f32;3]>, Vec<[f32;3]>, Vec<[f32;2]>, Vec<u32>);
type MeshDataWithCoord = (ChunkCoord, MeshData);
