use bevy::prelude::*;

use crate::{player::PlayerPlugin, loader::WorldLoaderPlugin, grab_mouse, GameState};

use super::pause_menu::PauseMenuPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PlayerPlugin)
            .add_plugin(WorldLoaderPlugin)
            .add_plugin(PauseMenuPlugin)
            .add_startup_system(init_camera)
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(grab_mouse_system));
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn grab_mouse_system(mut windows: ResMut<Windows>) {
    grab_mouse(windows.get_primary_mut().unwrap());
}