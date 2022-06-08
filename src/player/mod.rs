use bevy::{prelude::*, utils::Instant};
use bevy_flycam::FlyCam;

use crate::loader::ChunkScanner;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_flycam::PlayerPlugin);
        app.add_system_to_stage(CoreStage::PreUpdate ,update_camera_pos);
        app.add_startup_system(setup);
        app.add_system(mining);
        app.add_system(building);
    }
}

fn setup(
    mut cmds: Commands,
) {
    cmds.insert_resource(Miner::default());
    cmds.insert_resource(Builder::default());
}

fn mining(
    mut miner: ResMut<Miner>,
    player: Query<&Player>
) {

}

fn building(
    mut builder: ResMut<Builder>
) {

}

fn update_camera_pos(
    mut scanner: Query<&mut ChunkScanner>,
    query: Query<&Transform, With<FlyCam>>,
) {
    let translation = query.get_single().unwrap().translation.clone();
    scanner.get_single_mut().unwrap().update(translation);
}

#[derive(Component)]
struct Player;

struct Miner {
    mining_progress: f32,
    coord: [i32;3],
    last_time: Instant,
}

impl Default for Miner {
    fn default() -> Self {
        Self { mining_progress: Default::default(), coord: Default::default(), last_time: Instant::now() }
    }
}

struct Builder(Instant);

impl Miner {
    pub fn reset_miner(&mut self, coord: [i32;3]) {
        self.mining_progress = 0.0;
        self.coord = coord;
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