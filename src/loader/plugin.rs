use bevy::{prelude::*, math::ivec3, render::{render_resource::FilterMode, texture::ImageSampler}, ecs::event::Events};use bevy::render::texture::ImageSampler::Descriptor;
use bevy_atmosphere::prelude::AtmospherePlugin;
use futures_lite::future;

use crate::{player::{Player, Gravity}, GameState};

use super::{Worldgen, texture::{TextureMapHandle, load_texture_map_info, TextureMapInfo}, ChunkScanner, ChunkBuildTask};

pub struct WorldLoaderPlugin;

impl Plugin for WorldLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AtmospherePlugin);

        app.add_system_set(SystemSet::on_enter(GameState::Game).label("Setup").with_system(setup));
        app.add_system_set(SystemSet::on_enter(GameState::Game).label("PostSetup").after("Setup").with_system(load_texture_map_info));
        app.add_system_set(SystemSet::on_update(GameState::Game).label("Update").with_system(scan_chunks).with_system(queue_mesh_rebuild).with_system(build_chunks).with_system(build_meshes));
        app.add_system_set(SystemSet::on_update(GameState::Game).label("PreUpdate").before("Update").with_system(unload_chunks).with_system(unload_meshes));
        app.add_system_set(SystemSet::on_update(GameState::Game).label("PostUpdate").after("Update").with_system(when_texture_loads).with_system(start_gravity));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(Worldgen::new(0));
    let texture_handle: Handle<Image> = asset_server.load("map.dds");
    commands.insert_resource(TextureMapHandle(texture_handle));

    let render_distance = RenderDistance::default();
    commands.spawn(ChunkScanner::new(1u16 + render_distance.get() as u16, ivec3(0,0,0)));
    commands.insert_resource(render_distance);

    let mut rot = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_3);
    rot = rot.mul_quat(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_6));
    
    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: rot,
            ..default()
        },
        ..default()
    });
}

fn scan_chunks(
    scanner: Query<&mut ChunkScanner>,
    mut worldgen: ResMut<Worldgen>,
    commands: Commands
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

#[derive(Clone, Copy, Resource)]
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

impl From<RenderDistance> for u32 {
    fn from(render_distance: RenderDistance) -> Self {
        render_distance.0
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

fn start_gravity(
    query: Query<Entity, With<Player>>,
    worldgen: Res<Worldgen>,
    mut commands: Commands
) {
    if worldgen.loaded_chunk_count() > 5000 {
        commands.entity(query.single()).insert(Gravity);
    }
}