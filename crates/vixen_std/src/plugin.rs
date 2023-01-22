use bevy::prelude::Plugin;
use vixen_core::loader::register_biome;

use crate::biomes::ForestBiome;
pub struct StandardPlugin;

impl Plugin for StandardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(register_everything);
    }
}

fn register_everything() {
    register_biome(ForestBiome);
}