use bevy::{prelude::*, diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}};
use bevy_egui::EguiContext;

use crate::{player::Player, physics::Movement};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(player_ui);
        app.add_system(perf_stats);
    }
}

fn player_ui(
    mut egui_ctx: ResMut<EguiContext>,
    player_query: Query<(&Transform, &Movement), With<Player>>,
    camera_query: Query<&Transform, With<Camera3d>>,
) {
    let (transform, movement) = player_query.single();
    egui::Window::new("Player Debug Info")
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label(format!("Position: {}", transform.translation));
            ui.label(format!("Velocity: {}", movement.velocity));
            ui.label(format!("Facing: {}", camera_query.single().forward()));
            ui.set_min_width(50.);
            ui.set_max_width(250.);
        });
}

fn perf_stats(
    mut egui_ctx: ResMut<EguiContext>,
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
) {
    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average());

    egui::Window::new("Perf").show(egui_ctx.ctx_mut(), |ui| {
        if let Some(fps) = fps {
            ui.label(format!("FPS: {fps}"));
        }
        ui.label(format!("Delta (ms): {}", time.delta_seconds() * 1000.));
    });
    
}