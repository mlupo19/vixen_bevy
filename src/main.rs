use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use debug::DebugPlugin;
use game::GamePlugin;
use menu::MenuPlugin;

mod loader;
mod player;
mod menu;
mod game;
mod terrain;
mod storage;
mod physics;
mod debug;
mod util;

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    MainMenu,
    Game,
}

fn main() {
    App::new()
        .add_system(bevy::window::close_on_esc)
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(MenuPlugin)
        .add_state(GameState::MainMenu)
        .add_plugin(EguiPlugin)
        .add_plugin(DebugPlugin)
        .insert_resource(Msaa { samples: 4 })
        .run();    
}