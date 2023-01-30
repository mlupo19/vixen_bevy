mod plugin;

pub use plugin::*;

use bevy::prelude::*;


pub fn spawn_button(
    commands: &mut Commands,
    menu_font: &MenuFont,
    text: &str,
) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(400.0), Val::Px(150.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(100.0)),
                ..Default::default()
            },
            background_color: BackgroundColor(NORMAL_BUTTON),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: menu_font.0.clone(),
                        font_size: 40.0,
                        color: Color::rgb(0.95, 0.95, 0.95),
                    },
                ),
                ..default()
            });
        })
        .id()
}
