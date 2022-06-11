use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};
use bevy_flycam::MovementSettings;
use loader::WorldLoaderPlugin;
use player::PlayerPlugin;

mod loader;
mod player;

fn main() {
    App::new()
        .add_startup_system(init_settings)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldLoaderPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .insert_resource(Msaa { samples: 4 })
        .run();    
}

fn init_settings(
    mut move_settings: ResMut<MovementSettings>,
    mut windows: ResMut<Windows>,
) {
    move_settings.speed = 10.0;
    windows.get_primary_mut().unwrap().set_resolution(1920.0, 1080.0);
}