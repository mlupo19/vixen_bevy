mod scanner;
mod chunk;
mod generator;
mod texture;
mod worldgen;

use bevy::{prelude::*, utils::{HashMap, HashSet}, tasks::{AsyncComputeTaskPool, Task}, math::{ivec3, vec3}, render::render_resource::FilterMode, ecs::event::Events};
use chunk::*;
use futures_lite::future;

use ndarray::Array3;
pub use scanner::ChunkScanner;

use self::{generator::TerrainGenerator, texture::TextureMapInfo};

pub type ChunkCoord = IVec3;
pub type BlockCoord = IVec3;

#[derive(Component)]
struct ChunkBuildTask(pub Task<(ChunkCoord, Chunk)>);

#[derive(Component)]
struct MeshBuildTask(pub Task<MeshDataWithCoord>);

#[derive(Component)]
struct NeedsMeshBuild(pub HashSet<ChunkCoord>);

#[derive(Component)]
struct NeedsChunkBuild(pub HashSet<ChunkCoord>);

type MeshData = (Vec<[f32;3]>, Vec<[f32;3]>, Vec<[f32;2]>, Vec<u32>);
type MeshDataWithCoord = (ChunkCoord, MeshData);

pub struct WorldLoaderPlugin;

impl Plugin for WorldLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.add_system_to_stage(CoreStage::PreUpdate, scan_chunks);
        app.add_system_to_stage(CoreStage::PreUpdate, queue_mesh_rebuild);
        app.add_system_to_stage(CoreStage::PreUpdate, build_chunks);
        app.add_system_to_stage(CoreStage::PreUpdate, build_meshes);
        app.add_stage_before(CoreStage::PreUpdate, "Unload", SystemStage::parallel());
        app.add_system_to_stage("Unload", unload_chunks);
        app.add_system_to_stage(CoreStage::PreUpdate, unload_meshes);
        app.add_system(when_texture_loads);
        app.insert_resource(HashMap::<ChunkCoord, Chunk>::new());
        app.insert_resource(HashMap::<ChunkCoord, Handle<Mesh>>::new());
        app.insert_resource(NeedsMeshBuild(HashSet::new()));
        app.insert_resource(NeedsChunkBuild(HashSet::new()));
        app.insert_resource(TerrainGenerator::new(0));

    }
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture_handle = asset_server.load("map.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        ..default()
    });

    let render_distance = RenderDistance::default();
    commands.spawn().insert(ChunkScanner::new(1u16 + render_distance.get() as u16, ivec3(0,0,0)));
    commands.insert_resource(render_distance);
    commands.insert_resource(material_handle);
    commands.insert_resource(texture::load_texture_map_info((512,512)));

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
    chunk_map: Res<HashMap<ChunkCoord, Chunk>>,
    pool: Res<AsyncComputeTaskPool>,
    generator: Res<TerrainGenerator>,
    mut needs_build: ResMut<NeedsChunkBuild>,
    mut commands: Commands,
) {
    for scanner in scanner.iter() {
        for chunk_coord in scanner.into_iter() {
            if !chunk_map.contains_key(&chunk_coord) && !needs_build.0.contains(&chunk_coord) {
                needs_build.0.insert(chunk_coord.clone());
                let generator = generator.clone();
                let task = pool.spawn(async move {
                    generator.gen(chunk_coord)
                });
                commands.spawn().insert(ChunkBuildTask(task));
            }
        }
    }
}

fn build_chunks(
    mut chunk_map: ResMut<HashMap<ChunkCoord, Chunk>>,
    mut tasks: Query<(Entity, &mut ChunkBuildTask)>,
    mut to_build: ResMut<NeedsMeshBuild>,
    mut needs_build: ResMut<NeedsChunkBuild>,
    mut commands: Commands,
) {
    tasks.for_each_mut(|(entity, mut task)| {
        if let Some((coord, chunk)) = future::block_on(future::poll_once(&mut task.0)) {
            chunk_map.insert(coord.clone(), chunk);
            commands.entity(entity).remove::<ChunkBuildTask>();
            to_build.0.insert(coord);
            needs_build.0.remove(&coord);
        }
    });
}

fn queue_mesh_rebuild(
    chunk_map: Res<HashMap<ChunkCoord, Chunk>>,
    mesh_map: Res<HashMap<ChunkCoord, Handle<Mesh>>>,
    scanner: Query<&ChunkScanner>,
    mut to_build: ResMut<NeedsMeshBuild>,
) {
    for (coord, chunk) in chunk_map.iter() {
        if chunk.needs_update() || (scanner.single().should_load_mesh(coord) && !mesh_map.contains_key(coord)) {
            to_build.0.insert(*coord);
        }
    }
}

fn build_meshes(
    pool: Res<AsyncComputeTaskPool>,
    texture_map_info: Res<TextureMapInfo>,
    material_handle: Res<Handle<StandardMaterial>>,
    scanner: Query<&ChunkScanner>,
    mut chunk_map: ResMut<HashMap<ChunkCoord, Chunk>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_map: ResMut<HashMap<ChunkCoord, Handle<Mesh>>>,
    mut commands: Commands,
    mut to_build: ResMut<NeedsMeshBuild>,
) {
    let task = pool.scope(|scope| {
        to_build.0.drain_filter(|coord| {
            if let Some(chunk) = chunk_map.get(&coord) {
                if !chunk.is_empty() && scanner.single().should_load_mesh(coord) {
                    if let Some(neighbors) = get_neighbors(&chunk_map, *coord) {
                        let info = &texture_map_info.info;
                        let data = chunk.get_data().as_ref().unwrap();
                        let coord = coord.clone();
                        scope.spawn(async move {
                            let (vertices, normals, texture_coords, indices) = Chunk::gen_mesh(&data, neighbors, info);
                            let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList);
                            mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
                            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
                            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, texture_coords);

                            (coord, mesh)
                        });

                        return true;
                    }
                }
            }
            false
        });
    });

    for (coord, mesh) in task {
        if let Some(mesh_handle) = mesh_map.get(&coord) {
            meshes.remove(mesh_handle.id);
        }
        let mesh_handle = meshes.add(mesh);
        mesh_map.insert(coord, mesh_handle.clone());
        commands.spawn_bundle(MaterialMeshBundle {
            mesh: mesh_handle,
            material: material_handle.as_ref().clone(),
            transform: Transform::from_xyz(coord.x as f32 * CHUNK_SIZE.0 as f32, coord.y as f32 * CHUNK_SIZE.1 as f32, coord.z as f32 * CHUNK_SIZE.2 as f32),
            ..Default::default()
        });
        chunk_map.get_mut(&coord).unwrap().set_updated();
    }
}

fn unload_chunks(
    mut chunk_map: ResMut<HashMap<IVec3, Chunk>>,
    scanner: Query<&ChunkScanner>,
) {
    chunk_map.drain_filter(|coord, _chunk| {
        scanner.single().should_unload_chunk(coord)
    });
}

fn unload_meshes(
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_map: ResMut<HashMap<ChunkCoord, Handle<Mesh>>>,
    scanner: Query<&ChunkScanner>,
) {
    mesh_map.drain_filter(|coord, _mesh| {
        !scanner.single().should_load_mesh(coord)
    }).into_iter().for_each(|(_, mesh)| {
        meshes.remove(mesh.id);
    });
}

fn get_neighbors<'a>(chunk_map: &'a HashMap<IVec3, Chunk>, coord: IVec3) -> Option<[Option<&'a Box<Array3<Block>>>;6]> {
    Some([
        match chunk_map.get(&(ivec3(1,0,0) + coord)) {
            None => return None,
            Some(chunk) => chunk.get_data().as_ref(),
        },
        match chunk_map.get(&(ivec3(-1,0,0) + coord)) {
            None => return None,
            Some(chunk) => chunk.get_data().as_ref(),
        },
        match chunk_map.get(&(ivec3(0,-1,0) + coord)) {
            None => return None,
            Some(chunk) => chunk.get_data().as_ref(),
        },
        match chunk_map.get(&(ivec3(0,1,0) + coord)) {
            None => return None,
            Some(chunk) => chunk.get_data().as_ref(),
        },
        match chunk_map.get(&(ivec3(0,0,1) + coord)) {
            None => return None,
            Some(chunk) => chunk.get_data().as_ref(),
        },
        match chunk_map.get(&(ivec3(0,0,-1) + coord)) {
            None => return None,
            Some(chunk) => chunk.get_data().as_ref(),
        },
    ])
}

#[derive(Clone, Copy)]
struct RenderDistance(u32);

impl Default for RenderDistance {
    fn default() -> Self {
        Self(10)
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
                texture.get_mut(handle).unwrap().sampler_descriptor.min_filter = FilterMode::Linear;
                texture.get_mut(handle).unwrap().sampler_descriptor.anisotropy_clamp = std::num::NonZeroU8::new(16);
                texture.get_mut(handle).unwrap().sampler_descriptor.mipmap_filter = FilterMode::Linear;
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

pub fn get_block<'a>(chunk_map: &'a HashMap<IVec3, Chunk>, coord: &BlockCoord) -> Option<Block> {
    let (x,y,z) = (coord.x,coord.y,coord.z);
    let chunk_coord = ivec3(
        (x as f32 / CHUNK_SIZE.0 as f32).floor() as i32,
        (y as f32 / CHUNK_SIZE.1 as f32).floor() as i32,
        (z as f32 / CHUNK_SIZE.2 as f32).floor() as i32,
    );
    match chunk_map.get(&chunk_coord) {
        None => None,
        Some(chunk) => chunk.get_block((
            (x - chunk_coord.x * CHUNK_SIZE.0 as i32) as usize,
            (y - chunk_coord.y * CHUNK_SIZE.1 as i32) as usize,
            (z - chunk_coord.z * CHUNK_SIZE.2 as i32) as usize,
        )),
    }
}

pub fn set_block<'a>(chunk_map: &'a mut HashMap<IVec3, Chunk>, coord: &BlockCoord, block: Block) -> Result<(), ()> {
    let (x,y,z) = (coord.x,coord.y,coord.z);
    let chunk_coord = ivec3(
        (x as f32 / CHUNK_SIZE.0 as f32).floor() as i32,
        (y as f32 / CHUNK_SIZE.1 as f32).floor() as i32,
        (z as f32 / CHUNK_SIZE.2 as f32).floor() as i32,
    );
    match chunk_map.get_mut(&chunk_coord) {
        None => Err(()),
        Some(chunk) => {
            chunk.set_block((
                (x - chunk_coord.x * CHUNK_SIZE.0 as i32) as usize,
                (y - chunk_coord.y * CHUNK_SIZE.1 as i32) as usize,
                (z - chunk_coord.z * CHUNK_SIZE.2 as i32) as usize,
            ), block);
            Ok(())
        },
    }
}