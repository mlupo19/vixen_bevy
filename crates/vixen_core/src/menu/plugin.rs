use bevy::prelude::*;

use crate::GameState;

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
    asset_server: Res<AssetServer>,
) {
    let start_button = spawn_button(&mut commands, &asset_server, "Start");
    commands.entity(start_button).insert(MainMenuButton::StartButton);

    let exit_button = spawn_button(&mut commands, &asset_server, "Exit");
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

fn spawn_button(
    commands: &mut Commands,
    asset_server: &AssetServer,
    text: &str,
) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::rgb(1.0, 0.1, 0.1)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.95, 0.95, 0.95),
                    },
                ),
                ..default()
            });
        })
        .id()
}