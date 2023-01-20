mod scanner;
mod chunk;
mod texture;
mod worldgen;
mod block_data;
mod tool_data;
mod plugin;

use bevy::{prelude::*, utils::HashSet, tasks::{AsyncComputeTaskPool, Task}};

pub use scanner::ChunkScanner;
pub use worldgen::Worldgen;
pub use worldgen::ChunkMap;
pub use worldgen::UnfinishedChunkData;
pub use chunk::*;
pub use plugin::*;

use crate::util::ChunkCoord;

use self::texture::{TextureMapInfo, TextureMapHandle};

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
