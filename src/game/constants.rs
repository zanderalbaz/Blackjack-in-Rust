use bevy::prelude::*;

pub static NO_CARD_VALUE: u8 = u8::MAX;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Start,
    InGame,
}

