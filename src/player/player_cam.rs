use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::physics::Movement;

use super::{Player, Jumper};

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

/// Mouse sensitivity and movement speed
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
    pub jump_power: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 4.317,
            jump_power: 6.,
        }
    }
}

/// A marker component used in queries when you want player cams and not other cameras
#[derive(Component)]
pub struct PlayerCam(pub Entity);

impl PlayerCam {
    pub fn get(&self) -> Entity {
        self.0
    }
}

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
}

/// Grabs the cursor when game first starts
fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        toggle_grab_cursor(window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

/// Handles keyboard input and movement
fn player_input(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Res<Windows>,
    settings: Res<MovementSettings>,
    mut player_query: Query<(&mut Transform, &mut PlayerCam, &mut Movement, &mut Jumper), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    if let Some(window) = windows.get_primary() {
        for (mut transform, cam, mut movement, mut jumper) in player_query.iter_mut() {
            let cam_transform = camera_query.get_mut(cam.get()).unwrap();
            let mut delta = Vec3::ZERO;
            transform.rotation = cam_transform.rotation;
            
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z).normalize();
            let right = Vec3::new(local_z.z, 0., -local_z.x).normalize();

            for key in keys.get_pressed() {
                if window.cursor_locked() {
                    match key {
                        KeyCode::W => delta += forward,
                        KeyCode::S => delta += -forward,
                        KeyCode::A => delta += -right,
                        KeyCode::D => delta += right,
                        KeyCode::Space => if jumper.0 {
                            movement.velocity += Vec3::Y * settings.jump_power;
                            jumper.0 = false;
                        },
                        _ => (),
                    }
                }
            }

            movement.delta += delta * settings.speed * time.delta_seconds();
        }
    } else {
        warn!("Primary window not found for `player_input`!");
    }
}

/// Handles looking around if cursor is locked
fn player_look(
    settings: Res<MovementSettings>,
    windows: Res<Windows>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    if let Some(window) = windows.get_primary() {
        let mut delta_state = state.as_mut();
        for mut transform in query.iter_mut() {
            for ev in delta_state.reader_motion.iter(&motion) {
                if window.cursor_locked() {
                    // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                    let window_scale = window.height().min(window.width());
                    delta_state.pitch -=
                        (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                    delta_state.yaw -=
                        (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                }

                delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
                    * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        if keys.just_pressed(KeyCode::LControl) {
            toggle_grab_cursor(window);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

fn lock_cursor_position(mut windows: ResMut<Windows>) {
	if let Some(window) = windows.get_primary_mut() {
        if window.cursor_locked() {
		    window.set_cursor_position(Vec2::new(window.width() / 2., window.height() / 2.));
        }
	}
}

/// Contains everything needed to add first-person camera behavior to your game
pub struct PlayerCameraPlugin;
impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .add_startup_system(initial_grab_cursor)
            .add_system_to_stage(CoreStage::PreUpdate, player_input)
            .add_system_to_stage(CoreStage::PreUpdate,player_look)
            .add_system_to_stage(CoreStage::PreUpdate,cursor_grab)
            .add_system(lock_cursor_position);
    }
}