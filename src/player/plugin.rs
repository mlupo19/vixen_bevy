use bevy::{prelude::*, math::{ivec3, vec3}};

use crate::{loader::{ChunkScanner, Worldgen, BlockCoord, block_data::get_durability}, storage::StorageContainer, physics::{Movement, AABB, SweptCollider}};
use crate::player::Block;

use super::{Builder, Miner, Player, player_cam::{PlayerCameraPlugin, PlayerCam}, PlayerBundle, Jumper, Gravity};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerCameraPlugin);
        app.add_system(mine);
        app.add_system(build);
        app.add_system(update_scanner);
        app.add_system(player_move);
        app.add_startup_system_to_stage(StartupStage::PostStartup, setup);
    }
}

fn setup(
    mut commands: Commands,
) {
    let cam = commands.spawn().insert_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).id();

    commands.spawn_bundle(PlayerBundle {
        transform: Transform::from_translation(Vec3::new(0.0,100.0,0.0)),
        movement: Movement::default(),
        miner: Miner::default(),
        builder: Builder::default(),
        storage: StorageContainer::new(32),
        camera: PlayerCam(cam),
        player: Player::default(),
        jumper: Jumper(false),
    });
}

fn mine(
    mouse_input: Res<Input<MouseButton>>,
    time: Res<Time>,
    camera_transform: Query<&Transform, With<Camera3d>>,
    mut worldgen: ResMut<Worldgen>,
    mut query: Query<&mut Miner, With<Player>>,
) {
    let mut miner = query.single_mut();
    let transform = camera_transform.single();
    
    // Check if player is trying to mine
    if mouse_input.pressed(MouseButton::Left) {
        let range = 4.0;
        let coord = cast_ray(transform.translation, range, transform.forward(), &worldgen);
        miner.mine(&coord, time.delta().as_secs_f32(), 10.0, &mut worldgen);
    }
}

fn build(
    mouse_input: Res<Input<MouseButton>>,
    camera_transform: Query<&Transform, With<Camera3d>>,
    mut worldgen: ResMut<Worldgen>,
    mut query: Query<&mut Builder>
) {
    let transform = camera_transform.single();
    let mut builder = query.single_mut();

    // Check if player is trying to build
    if mouse_input.just_pressed(MouseButton::Right) || (mouse_input.pressed(MouseButton::Right) && builder.can_build()) {
        builder.update();
        let range = 5.0;
        let translation = &transform.translation;
        let coord = cast_ray_in_front(vec3(translation.x, translation.y, translation.z), range, transform.forward(), &worldgen);
        if let Some(coord) = coord {
            if coord != ivec3(translation.x.floor() as i32, translation.y.floor() as i32, translation.z.floor() as i32)
            && coord != ivec3(translation.x.floor() as i32, translation.y.floor() as i32 - 1, translation.z.floor() as i32) {
                worldgen.set_block(&coord, Block::new(1, get_durability(1)));
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
            }
        }
    }
    ivec3(start_point[0].floor() as i32, start_point[1].floor() as i32, start_point[2].floor() as i32)
}

/// Casts a ray and returns block coordinate of the air block in front of the block the ray hit, and None otherwise
fn cast_ray_in_front(start_point: Vec3, rho: f32, forward: Vec3, loader: &Worldgen) -> Option<BlockCoord> {
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

fn update_scanner(
    camera_transform: Query<&Transform, With<Camera3d>>,
    mut scanner: Query<&mut ChunkScanner>,
) {
    let transform = camera_transform.single();
    scanner.get_single_mut().unwrap().update(transform.translation);
}

fn player_move(
    time: Res<Time>,
    worldgen: Res<Worldgen>,
    mut query: Query<(&mut Transform, &mut Movement, &mut Jumper), (With<Player>, With<Gravity>)>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    if let Ok((mut transform, mut movement, mut jumper)) = query.get_single_mut() {
        movement.velocity -= Vec3::Y * 16. * time.delta_seconds();
        let velo = movement.velocity;
        movement.delta += velo * time.delta_seconds();

        resolve_collision(worldgen, &mut movement, AABB::from_player(transform.translation), &mut jumper);

        transform.translation += movement.delta;
        movement.delta = Vec3::ZERO;
        camera_query.single_mut().translation = transform.translation + Vec3::new(0.0,1.5,0.0);
    }
}

fn resolve_collision(worldgen: Res<Worldgen>, movement: &mut Movement, aabb: AABB, jumper: &mut Jumper) {
    let extended = aabb.extend(&movement.delta);
    let nearby_blocks: Vec<IVec3> = get_nearby_blocks(worldgen, &extended).collect();
    
    let mut collisions_remain = true;
    while collisions_remain {
        collisions_remain = false;

        let (mut nearest_collision, mut nearest_collision_tangent) = (1.0, Vec3::ZERO);

        for block in &nearby_blocks {
            let block_aabb = AABB::from_block(block);
            if let Some((collision_time, tangent)) = aabb.swept_collide(&block_aabb, movement) {
                if collision_time < nearest_collision {
                    nearest_collision = collision_time;
                    nearest_collision_tangent = tangent;
                }
                collisions_remain = true;
            }
        }
        
        // Reset velocity when hit block
        if collisions_remain {
            if nearest_collision_tangent.x == 0.0 {
                movement.velocity.x = 0.0;
            }
            if nearest_collision_tangent.y == 0.0 {
                movement.velocity.y = 0.0;

                // Only reset jump if we hit the ground
                if movement.delta.y.signum() < 0.0 {
                    jumper.0 = true;
                }
            }
            if nearest_collision_tangent.z == 0.0 {
                movement.velocity.z = 0.0;
            }
        }

        // Stop movement when hit block
        movement.delta *= nearest_collision;


        // Slide against wall
        let remaining_time = 1.0 - nearest_collision;
        movement.delta += remaining_time * nearest_collision_tangent;
    }
}

fn get_nearby_blocks<'a>(worldgen: Res<'a, Worldgen>, aabb: &AABB) -> impl Iterator<Item = IVec3> + 'a {
    let (x_min, x_max) = (aabb.min.x.floor() as i32 - 1, aabb.max.x.ceil() as i32 + 1);
    let (y_min, y_max) = (aabb.min.y.floor() as i32 - 1, aabb.max.y.ceil() as i32 + 1);
    let (z_min, z_max) = (aabb.min.z.floor() as i32 - 1, aabb.max.z.ceil() as i32 + 1);

    (x_min..x_max).flat_map(
        move |x| (y_min..y_max).flat_map(
            move |y| (z_min..z_max).map(
                move |z| ivec3(x,y,z)
            )
        )
     ).filter(move |coord| !worldgen.get_block(coord).unwrap_or(Block::air()).is_air())
}