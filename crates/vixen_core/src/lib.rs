pub mod loader;
pub mod player;
pub mod main_menu;
pub mod game;
pub mod terrain;
pub mod storage;
pub mod physics;
pub mod debug;
pub mod util;
pub mod ui;

pub use game::GamePlugin;
pub use main_menu::MenuPlugin;
pub use debug::DebugPlugin;
pub use util::*;
pub use loader::Block;

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    MainMenu,
    Game,
}