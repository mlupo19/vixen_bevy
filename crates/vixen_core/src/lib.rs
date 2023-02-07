pub mod debug;
pub mod game;
pub mod loader;
pub mod main_menu;
pub mod physics;
pub mod player;
pub mod storage;
pub mod terrain;
pub mod ui;
pub mod util;

pub use debug::DebugPlugin;
pub use game::GamePlugin;
pub use loader::Block;
pub use main_menu::MenuPlugin;
pub use util::*;

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    MainMenu,
    Game,
}
