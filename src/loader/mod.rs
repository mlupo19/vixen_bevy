mod scanner;
mod chunk;
mod generator;
mod texture;
mod worldgen;
pub mod block_data;
mod tool_data;

use bevy::{prelude::*, utils::HashSet, tasks::{AsyncComputeTaskPool, Task}, math::{ivec3, vec3}, render::{render_resource::{FilterMode, Texture}, texture::ImageSampler}, ecs::event::Events};use bevy::render::texture::ImageSampler::Descriptor;
use chunk::*;
use futures_lite::future;

use ndarray::Array3;
pub use scanner::ChunkScanner;
pub use worldgen::Worldgen;
pub use worldgen::ChunkMap;
pub use chunk::*;

use crate::player::{Player, Gravity};

use self::{generator::TerrainGenerator, texture::{TextureMapInfo, TextureMapHandle, load_texture_map_info}, block_data::load_block_data};

pub type ChunkCoord = IVec3;
pub type BlockCoord = IVec3;

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

pub struct WorldLoaderPlugin;

impl Plugin for WorldLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup);
        app.add_startup_system_to_stage(StartupStage::Startup, load_texture_map_info);
        app.add_system_to_stage(CoreStage::PreUpdate, scan_chunks);
        app.add_system_to_stage(CoreStage::PreUpdate, queue_mesh_rebuild);
        app.add_system_to_stage(CoreStage::PreUpdate, build_chunks);
        app.add_system_to_stage(CoreStage::PreUpdate, build_meshes);
        app.add_stage_before(CoreStage::PreUpdate, "Unload", SystemStage::parallel());
        app.add_system_to_stage("Unload", unload_chunks);
        app.add_system_to_stage(CoreStage::PreUpdate, unload_meshes);
        app.add_system(when_texture_loads);
        app.add_system(start_gravity);
        app.insert_resource(Worldgen::new(0));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture_handle: Handle<Image> = asset_server.load("map.dds");
    commands.insert_resource(TextureMapHandle(texture_handle));

    let render_distance = RenderDistance::default();
    commands.spawn().insert(ChunkScanner::new(1u16 + render_distance.get() as u16, ivec3(0,0,0)));
    commands.insert_resource(render_distance);

    let mut rot = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_3);
    rot = rot.mul_quat(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_6));
    // light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 10000.0, 0.0),
            rotation: rot,
            ..default()
        },
        ..default()
    });
}

fn scan_chunks(
    scanner: Query<&ChunkScanner>,
    commands: Commands,
    mut worldgen: ResMut<Worldgen>,
) {
    worldgen.scan_chunks(scanner, commands);
}

fn build_chunks(
    mut tasks: Query<(Entity, &mut ChunkBuildTask)>,
    mut commands: Commands,
    mut worldgen: ResMut<Worldgen>,
) {
    tasks.for_each_mut(|(entity, mut task)| {
        if let Some((coord, chunk)) = future::block_on(future::poll_once(&mut task.0)) {
            worldgen.build_chunk(coord, chunk);
            commands.entity(entity).remove::<ChunkBuildTask>();
        }
    });
    
}

fn queue_mesh_rebuild(
    mut worldgen: ResMut<Worldgen>,
    scanner: Query<&ChunkScanner>,
) {
    worldgen.queue_mesh_rebuild(scanner);
}

fn build_meshes(
    scanner: Query<&ChunkScanner>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    commands: Commands,
    texture_map: Res<TextureMapHandle>,
    texture_map_info: Res<TextureMapInfo>,
    mut worldgen: ResMut<Worldgen>,
) {
    worldgen.build_meshes(scanner, meshes, materials, commands, texture_map, texture_map_info);
}

fn unload_chunks(
    mut worldgen: ResMut<Worldgen>,
    scanner: Query<&ChunkScanner>,
) {
    worldgen.unload_chunks(scanner);
}

fn unload_meshes(
    scanner: Query<&ChunkScanner>,
    meshes: ResMut<Assets<Mesh>>,
    mut worldgen: ResMut<Worldgen>,
) {
    worldgen.unload_meshes(scanner, meshes);
}

#[derive(Clone, Copy)]
struct RenderDistance(u32);

impl Default for RenderDistance {
    fn default() -> Self {
        Self(12)
    }
}

impl RenderDistance {
    pub fn set(&mut self, value: u32) {
        self.0 = value;
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

impl Into<u32> for RenderDistance {
    fn into(self) -> u32 {
        self.0
    }
}

fn when_texture_loads(
    events: Res<Events<AssetEvent<Image>>>,
    mut texture: ResMut<Assets<Image>>,
) {
    for event in events.get_reader().iter(&events) {
        match event {
            AssetEvent::Created { handle } => {
                texture.get_mut(handle).unwrap().sampler_descriptor = ImageSampler::nearest();
                if let Descriptor(ref mut desc) = texture.get_mut(handle).unwrap().sampler_descriptor {
                    desc.min_filter = FilterMode::Linear;
                    desc.anisotropy_clamp = std::num::NonZeroU8::new(16);
                    desc.mipmap_filter = FilterMode::Linear;
                    desc.mag_filter = FilterMode::Nearest;
                }
            },
            AssetEvent::Modified { handle: _ } => (),
            AssetEvent::Removed { handle: _ } => (),
        }
    }
}

fn to_chunk_coord(world_coord: &Vec3) -> ChunkCoord {
    ivec3((world_coord.x / CHUNK_SIZE.0 as f32).floor() as i32, (world_coord.y / CHUNK_SIZE.1 as f32).floor() as i32, (world_coord.z / CHUNK_SIZE.2 as f32).floor() as i32)
}

fn to_world_coord(chunk_coord: &ChunkCoord) -> Vec3 {
    vec3((chunk_coord.x * CHUNK_SIZE.0 as i32) as f32, (chunk_coord.y * CHUNK_SIZE.1 as i32) as f32, (chunk_coord.z * CHUNK_SIZE.2 as i32) as f32)
}

fn start_gravity(
    query: Query<Entity, With<Player>>,
    worldgen: Res<Worldgen>,
    mut commands: Commands
) {
    if worldgen.loaded_chunk_count() > 5000 {
        commands.entity(query.single()).insert(Gravity);
    }
}