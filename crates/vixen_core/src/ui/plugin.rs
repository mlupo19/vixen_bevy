use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_font);
        
        app.add_system(button_system);
    }
}

pub const NORMAL_BUTTON: Color = Color::rgb(0.3, 0.35, 0.45);
pub const HOVERED_BUTTON: Color = Color::rgb(0.4, 0.42, 0.5);
pub const PRESSED_BUTTON: Color = Color::rgb(0.17, 0.175, 0.225);

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, _children) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[derive(Resource)]
pub struct MenuFont(pub Handle<Font>);

fn load_font(
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {
    commands.insert_resource(MenuFont(asset_server.load("fonts/FiraSans-Bold.ttf")));
}