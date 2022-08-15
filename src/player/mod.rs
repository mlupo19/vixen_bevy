mod plugin;
pub mod player_cam;

use bevy::{utils::Instant, prelude::Component};
use bevy::prelude::*;
pub use plugin::PlayerPlugin;

use crate::loader::{BlockCoord, Worldgen};
use crate::loader::Block;
use crate::physics::Movement;
use crate::storage::StorageContainer;

use self::player_cam::PlayerCam;

pub const PLAYER_SIZE: (f32,f32,f32) = (0.75,1.75,0.75);

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    transform: Transform,
    movement: Movement,
    miner: Miner,
    builder: Builder,
    storage: StorageContainer,
    camera: PlayerCam,
    player: Player,
}

#[derive(Component)]
struct Miner {
    mining_progress: f32,
    coord: IVec3,
    last_time: Instant,
}

impl Default for Miner {
    fn default() -> Self {
        Self { mining_progress: Default::default(), coord: Default::default(), last_time: Instant::now() }
    }
}

#[derive(Component)]
struct Builder(Instant);

impl Miner {
    pub fn mine(&mut self, coord: &BlockCoord, delta: f32, speed: f32, worldgen: &mut Worldgen) {
        if &self.coord != coord {
            self.reset_miner(coord);
        }
        self.coord = *coord;
        self.update();
        let block = worldgen.get_block(coord).unwrap_or(Block::air());
        let health = block.health;
        self.mining_progress += delta * speed;
        if health - self.mining_progress <= 0.0 && !block.is_air() {
            worldgen.set_block(coord, Block::air());
        } else {
            println!("{} health left, ID: {}", health - self.mining_progress, block.id);
        }
    }

    pub fn reset_miner(&mut self, coord: &BlockCoord) {
        self.mining_progress = 0.0;
        self.coord = *coord;
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        if (now - self.last_time).as_millis() > 80 {
            self.mining_progress = 0.0;
        }
        self.last_time = now;
    }
}

impl Builder {
    pub fn can_build(&mut self) -> bool {
        let now = Instant::now();
        if (now - self.0).as_millis() > 200 {
            self.0 = now;
            return true;
        }
        false
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self(Instant::now())
    }
}