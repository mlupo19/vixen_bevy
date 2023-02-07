use bevy::prelude::*;

use crate::{
    grab_mouse,
    player::Playing,
    release_mouse,
    ui::{spawn_button, MenuFont},
    GameState,
};

use super::PauseMenuState;

pub struct PauseMenuPlugin;

#[derive(Component)]
struct PauseMenuUIRoot;

#[derive(Component)]
enum PauseMenuButton {
    Resume,
    Settings,
    Exit,
    Quit,
}

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(PauseMenuState::Paused).with_system(show_pause_menu),
        )
        .add_system_set(SystemSet::on_update(PauseMenuState::Paused).with_system(update_pause_menu))
        .add_system_set(SystemSet::on_exit(PauseMenuState::Paused).with_system(teardown_pause_menu))
        .add_state(PauseMenuState::Unpaused)
        .add_system(pause_on_escape);
    }
}

fn show_pause_menu(
    mut commands: Commands,
    mut window: ResMut<Windows>,
    mut playing: ResMut<Playing>,
    menu_font: Res<MenuFont>,
) {
    release_mouse(window.get_primary_mut().unwrap());

    playing.0 = false;

    let resume_button = spawn_button(&mut commands, &menu_font, "Resume");
    commands
        .entity(resume_button)
        .insert(PauseMenuButton::Resume);

    let settings_button = spawn_button(&mut commands, &menu_font, "Settings");
    commands
        .entity(settings_button)
        .insert(PauseMenuButton::Settings);

    let exit_button = spawn_button(&mut commands, &menu_font, "Exit");
    commands.entity(exit_button).insert(PauseMenuButton::Exit);

    let quit_button = spawn_button(&mut commands, &menu_font, "Quit to Desktop");
    commands.entity(quit_button).insert(PauseMenuButton::Quit);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .insert(PauseMenuUIRoot)
        .add_child(resume_button)
        .add_child(settings_button)
        .add_child(exit_button)
        .add_child(quit_button);
}

fn update_pause_menu(
    mut pause_menu_state: ResMut<State<PauseMenuState>>,
    mut game_state: ResMut<State<GameState>>,
    interaction_query: Query<
        (&Interaction, &PauseMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, button) in interaction_query.iter() {
        match button {
            PauseMenuButton::Resume => {
                if let Interaction::Clicked = interaction {
                    pause_menu_state.set(PauseMenuState::Unpaused).unwrap();
                }
            }
            PauseMenuButton::Settings => {
                if let Interaction::Clicked = interaction {
                    pause_menu_state.set(PauseMenuState::Settings).unwrap();
                }
            }
            PauseMenuButton::Exit => {
                if let Interaction::Clicked = interaction {
                    pause_menu_state.set(PauseMenuState::Unpaused).unwrap();
                    game_state.set(GameState::MainMenu).unwrap();
                }
            }
            PauseMenuButton::Quit => {
                if let Interaction::Clicked = interaction {
                    std::process::exit(0);
                }
            }
        }
    }
}

fn teardown_pause_menu(
    mut commands: Commands,
    mut window: ResMut<Windows>,
    mut playing: ResMut<Playing>,
    root_query: Query<Entity, With<PauseMenuUIRoot>>,
) {
    for root in root_query.iter() {
        commands.entity(root).despawn_recursive();
    }

    grab_mouse(window.get_primary_mut().unwrap());

    playing.0 = true;
}

fn pause_on_escape(
    mut state: ResMut<State<PauseMenuState>>,
    game_state: Res<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if game_state.current() == &GameState::Game && keyboard_input.just_pressed(KeyCode::Escape) {
        if let &PauseMenuState::Paused = state.current() {
            state.set(PauseMenuState::Unpaused).unwrap();
        } else {
            state.set(PauseMenuState::Paused).unwrap();
        }
    }
}
