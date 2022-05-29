use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, ecs::event::Events, render::render_resource::{Texture, FilterMode}};
use loader::{WorldLoaderPlugin, ChunkScanner, BlockTextureMap};
use bevy_flycam::{PlayerPlugin, FlyCam, MovementSettings};

mod loader;

fn main() {
    App::new()
        .add_startup_system(init_settings)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system_to_stage(CoreStage::PreUpdate ,update_camera_pos)
        .add_plugin(WorldLoaderPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .insert_resource(Msaa { samples: 4 })
        .run();    
}

fn update_camera_pos(
    mut scanner: Query<&mut ChunkScanner>,
    query: Query<&Transform, With<FlyCam>>,
) {
    let translation = query.get_single().unwrap().translation.clone();
    scanner.get_single_mut().unwrap().update(translation);
}


fn init_settings(
    mut move_settings: ResMut<MovementSettings>,
    mut windows: ResMut<Windows>,
) {
    move_settings.speed = 50.0;
    windows.get_primary_mut().unwrap().set_resolution(2580.0, 1080.0);
}