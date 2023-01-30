use bevy::prelude::*;

use crate::{GameState, ui::{spawn_button, MenuFont}, release_mouse};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_menu))
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(update_menu))
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(teardown_menu));
    }
}

#[derive(Component)]
enum MainMenuButton {
    StartButton,
    ExitButton,
}

#[derive(Component)]
struct MenuUIRoot;

fn setup_menu(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    menu_font: Res<MenuFont>
) {
    commands.insert_resource(ClearColor(Color::rgb(0.75, 0.75, 0.75)));

    release_mouse(windows.get_primary_mut().unwrap());

    let start_button = spawn_button(&mut commands, &menu_font, "Start");
    commands.entity(start_button).insert(MainMenuButton::StartButton);

    let exit_button = spawn_button(&mut commands, &menu_font, "Exit");
    commands.entity(exit_button).insert(MainMenuButton::ExitButton);

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
        .insert(MenuUIRoot)
        .add_child(start_button)
        .add_child(exit_button);
}

fn update_menu(
    interaction_query: Query<
        (&Interaction, &MainMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<State<GameState>>,
    mut windows: ResMut<Windows>,
) {
    for (interaction, button) in interaction_query.iter() {
        match interaction {
            Interaction::Clicked => {
                match button {
                    MainMenuButton::StartButton => {
                        game_state.set(GameState::Game).unwrap();
                    }
                    MainMenuButton::ExitButton => {
                        windows.get_primary_mut().unwrap().close();
                    }
                }
            }
            _ => {}
        }
    }
}
fn teardown_menu(
    mut commands: Commands,
    root_query: Query<Entity, With<MenuUIRoot>>,
) {
    for root in root_query.iter() {
        commands.entity(root).despawn_recursive();
    }
}
