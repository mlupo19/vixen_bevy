use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_egui::EguiPlugin;
use debug::DebugPlugin;
use loader::WorldLoaderPlugin;
use player::PlayerPlugin;

mod loader;
mod player;
mod storage;
mod physics;
mod debug;
mod util;

fn main() {
    App::new()
        .add_startup_system(init_settings)
        .add_system(bevy::window::close_on_esc)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldLoaderPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(Msaa { samples: 4 })
        .run();    
}

fn init_settings(
    // mut windows: ResMut<Windows>,
) {
    // let window = windows.get_primary_mut().unwrap();
    // window.set_resolution(1920.0, 1080.0);
    // window.set_cursor_lock_mode(true);
    // window.set_cursor_visibility(false);
}