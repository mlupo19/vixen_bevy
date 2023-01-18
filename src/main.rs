use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_egui::EguiPlugin;
use debug::DebugPlugin;
use loader::WorldLoaderPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;

mod loader;
mod player;
mod storage;
mod physics;
mod debug;
mod util;
mod menu;

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    MainMenu,
    Game,
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PlayerPlugin)
            .add_plugin(WorldLoaderPlugin);
    }
}

fn main() {
    App::new()
        .add_startup_system(init_settings)
        .add_system(bevy::window::close_on_esc)
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(MenuPlugin)
        .add_state(GameState::MainMenu)
        .add_plugin(EguiPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(Msaa { samples: 4 })
        .run();    
}

fn init_settings(
    // mut windows: ResMut<Windows>,
    mut commands: Commands,
) {
    // let window = windows.get_primary_mut().unwrap();
    // window.set_resolution(1920.0, 1080.0);
    // window.set_cursor_lock_mode(true);
    // window.set_cursor_visibility(false);
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}