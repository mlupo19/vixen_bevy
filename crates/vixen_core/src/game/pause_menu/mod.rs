mod plugin;

pub use plugin::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PauseMenuState {
    Paused,
    Unpaused,
    Settings,
}