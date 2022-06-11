use bevy::{prelude::*, ecs::event::Events, utils::HashMap, math::{ivec3, vec3}, render::camera::Camera3d};
use bevy_flycam::FlyCam;

use crate::loader::{ChunkScanner, ChunkCoord, ChunkMap, Worldgen, BlockCoord};
use crate::player::Block;

use super::{Player, Builder, Miner};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_flycam::PlayerPlugin);
        app.add_system_to_stage(CoreStage::PreUpdate, update_camera_pos);
        app.add_system(update_player);
        app.add_startup_system_to_stage(StartupStage::PostStartup, setup);
    }
}

fn setup(
    mut query: Query<(Entity, &mut PerspectiveProjection)>,
    mut commands: Commands,
) {
    commands.get_or_spawn(query.single().0).insert(Builder::default()).insert(Miner::default());
    query.single_mut().1.set(Box::new(PerspectiveProjection {
        fov: std::f32::consts::PI / 3.0,
        near: 0.1,
        far: 1000.0,
        aspect_ratio: 16.0/9.0,
    })).unwrap();
}

fn update_camera_pos(
    mut scanner: Query<&mut ChunkScanner>,
    query: Query<&Transform, With<FlyCam>>,
) {
    let translation = query.single().translation.clone();
    scanner.get_single_mut().unwrap().update(translation);
}

fn update_player(
    mouse_input: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut worldgen: ResMut<Worldgen>,
    // mut player: ResMut<Player>,
    mut query: Query<(&Transform, &mut Builder, &mut Miner), With<FlyCam>>,
) {
    let (transform, mut builder, mut miner) = query.single_mut();
    
    // Check if player is trying to mine
    if mouse_input.pressed(MouseButton::Left) {
        let range = 4.0;
        let coord = cast_ray(transform.translation, range, transform.forward(), &worldgen);
        miner.mine(&coord, time.delta().as_secs_f32(), 10.0, &mut worldgen);
        println!("mining at {coord} from {}", transform.translation);
    }

    // Check if player is trying to build
    if mouse_input.pressed(MouseButton::Right) {
        if builder.can_build() {
            let range = 4.0;
            let translation = &transform.translation;
            let coord = cast_ray_in_front(vec3(translation.x, translation.y, translation.z), range, transform.forward(), &worldgen);
            if let Some(coord) = coord {
                if coord != ivec3(translation.x.floor() as i32, translation.y.floor() as i32, translation.z.floor() as i32)
                && coord != ivec3(translation.x.floor() as i32, translation.y.floor() as i32 - 1, translation.z.floor() as i32) {
                    worldgen.set_block(&coord, Block::new(1, 5.0));
                }
            }
        }
    }
}

fn cast_ray(start_point: Vec3, rho: f32, forward: Vec3, loader: &Worldgen) -> BlockCoord {
    let displacement = forward * rho;
    let end_point = (start_point[0] + displacement.x, start_point[1] + displacement.y, start_point[2] + displacement.z);

    for (x, y, z) in line_drawing::WalkVoxels::new((start_point[0], start_point[1], start_point[2]), end_point, &line_drawing::VoxelOrigin::Corner) {
        if let Some(block) = loader.get_block(&ivec3(x,y,z)) {
            if !block.is_air() {
                return ivec3(x,y,z);
            } else {
                println!("Block ({x} {y} {z}): {:?}", block);
            }
        }
    }
    println!("------");
    ivec3(start_point[0].floor() as i32, start_point[1].floor() as i32, start_point[2].floor() as i32)
}

/// Casts a ray and returns block coordinate of the air block in front of the block the ray hit, and None otherwise
fn cast_ray_in_front(start_point: Vec3, rho: f32, forward: Vec3, loader: &Worldgen) -> Option<BlockCoord>{
    let displacement = forward * rho;
    let end_point = (start_point[0] + displacement.x, start_point[1] + displacement.y, start_point[2] + displacement.z);
    let mut last = ivec3(start_point[0].floor() as i32, start_point[1].floor() as i32, start_point[2].floor() as i32);
    for (x, y, z) in line_drawing::WalkVoxels::new((start_point[0], start_point[1], start_point[2]), end_point, &line_drawing::VoxelOrigin::Corner) {
        let coord = ivec3(x, y, z);
        if let Some(block) = loader.get_block(&coord) {
            if block.id != 0 {
                return Some(last);
            }
        }
        last = ivec3(x,y,z);
    }
    None
}