use bevy::prelude::*;
use vixen_core::{ui::UiPlugin, DebugPlugin, GamePlugin, GameState, MenuPlugin};
use vixen_std::StandardPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(StandardPlugin)
        .add_state(GameState::MainMenu)
        .add_plugin(DebugPlugin)
        .insert_resource(Msaa { samples: 4 })
        .run();
}
