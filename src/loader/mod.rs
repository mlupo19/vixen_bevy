mod scanner;
mod chunk;
mod generator;
mod texture;

use bevy::{prelude::*, utils::{HashMap, HashSet}, tasks::{AsyncComputeTaskPool, Task}, math::ivec3, sprite::MaterialMesh2dBundle, render::render_resource::{Texture, FilterMode}, ecs::event::Events, asset::AssetLoader};
use chunk::*;
use futures_lite::future;

pub use scanner::ChunkScanner;

use self::{generator::TerrainGenerator, texture::TextureMapInfo};

#[derive(Component)]
struct ChunkBuildTask(pub Task<(IVec3, Chunk)>);

#[derive(Component)]
struct NeedsMeshBuild(pub HashSet<IVec3>);

#[derive(Component)]
struct NeedsChunkBuild(pub HashSet<IVec3>);

type MeshData = (Vec<[f32;3]>, Vec<[f32;3]>, Vec<[f32;2]>, Vec<u32>);
#[derive(Component)]
pub struct BlockTextureMap(pub Handle<Image>);

pub struct WorldLoaderPlugin;

impl Plugin for WorldLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.add_system_to_stage(CoreStage::PreUpdate, scan_chunks);
        app.add_system_to_stage(CoreStage::PreUpdate, build_chunks);
        app.add_system_to_stage(CoreStage::PreUpdate, build_meshes);
        app.add_system(when_texture_loads);
        app.insert_resource(HashMap::<IVec3, Chunk>::new());
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
    commands.insert_resource(BlockTextureMap(texture_handle.clone()));
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        ..default()
    });

    let render_distance = RenderDistance::default();
    commands.spawn().insert(ChunkScanner::new(render_distance.get() + 1, ivec3(0,0,0)));
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
    chunk_map: Res<HashMap<IVec3, Chunk>>,
    chunk_pool: Res<AsyncComputeTaskPool>,
    generator: Res<TerrainGenerator>,
    mut needs_build: ResMut<NeedsChunkBuild>,
    mut commands: Commands,
) {
    for scanner in scanner.iter() {
        for chunk_coord in scanner.into_iter() {
            if !chunk_map.contains_key(&chunk_coord) && !needs_build.0.contains(&chunk_coord) {
                needs_build.0.insert(chunk_coord.clone());
                let generator = generator.clone();
                let task = chunk_pool.spawn(async move {
                    generator.gen(chunk_coord.clone())
                });
                commands.spawn().insert(ChunkBuildTask(task));
            }
        }
    }
}

fn build_chunks(
    mut chunk_map: ResMut<HashMap<IVec3, Chunk>>,
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

fn build_meshes(
    material_handle: Res<Handle<StandardMaterial>>,
    texture_map_info: Res<TextureMapInfo>,
    chunk_map: Res<HashMap<IVec3, Chunk>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut to_build: ResMut<NeedsMeshBuild>,
    mut commands: Commands,
) {
    // TODO: Prevent allocation every frame
    let mut to_remove = vec![];
    for coord in &to_build.0 {
        let chunk = chunk_map.get(&coord).unwrap();
        if !chunk.is_empty() {
            if let Some(neighbors) = get_neighbors(&chunk_map, *coord) {
                let (vertices, normals, texture_coords, indices) = chunk.gen_mesh(neighbors, &texture_map_info.as_ref().info);
                let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList);
                mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, texture_coords);
                
                commands.spawn_bundle(MaterialMeshBundle {
                    mesh: meshes.add(mesh),
                    material: material_handle.as_ref().clone(),
                    transform: Transform::from_xyz(coord.x as f32 * CHUNK_SIZE.0 as f32, coord.y as f32 * CHUNK_SIZE.1 as f32, coord.z as f32 * CHUNK_SIZE.2 as f32),
                    ..Default::default()
                });

                to_remove.push(coord.clone());
            }
        }
    }    
    for coord in to_remove.drain(..) {
        to_build.0.remove(&coord);
    }                               
}

fn get_neighbors<'a>(chunk_map: &'a HashMap<IVec3, Chunk>, coord: IVec3) -> Option<[&'a Chunk;6]> {
    Some([
        match chunk_map.get(&(ivec3(1,0,0) + coord)) {
            None => return None,
            Some(chunk) => chunk,
        },
        match chunk_map.get(&(ivec3(-1,0,0) + coord)) {
            None => return None,
            Some(chunk) => chunk,
        },
        match chunk_map.get(&(ivec3(0,-1,0) + coord)) {
            None => return None,
            Some(chunk) => chunk,
        },
        match chunk_map.get(&(ivec3(0,1,0) + coord)) {
            None => return None,
            Some(chunk) => chunk,
        },
        match chunk_map.get(&(ivec3(0,0,1) + coord)) {
            None => return None,
            Some(chunk) => chunk,
        },
        match chunk_map.get(&(ivec3(0,0,-1) + coord)) {
            None => return None,
            Some(chunk) => chunk,
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
    map_handle: Res<BlockTextureMap>,
) {
    for event in events.get_reader().iter(&events) {
        match event {
            AssetEvent::Created { handle } => {
                if handle.clone() == map_handle.0 {
                    texture.get_mut(handle).unwrap().sampler_descriptor.min_filter = FilterMode::Linear;
                    texture.get_mut(handle).unwrap().sampler_descriptor.anisotropy_clamp = std::num::NonZeroU8::new(16);
                }
            },
            AssetEvent::Modified { handle: _ } => (),
            AssetEvent::Removed { handle: _ } => (),
        }
    }
}