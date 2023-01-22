use bevy::prelude::*;
use vixen_core::{GamePlugin, MenuPlugin, GameState, DebugPlugin};
use vixen_std::StandardPlugin;

fn main() {
    App::new()
        .add_system(bevy::window::close_on_esc)
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(StandardPlugin)
        .add_state(GameState::MainMenu)
        .add_plugin(DebugPlugin)
        .insert_resource(Msaa { samples: 4 })
        .run();    
}