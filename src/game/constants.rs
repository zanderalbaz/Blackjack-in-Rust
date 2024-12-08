use bevy::prelude::*;

pub static NO_CARD_VALUE: u8 = u8::MAX;
pub static CARD_HORIZONTAL_SPACING: f32 = 100.;
pub static DEALER_CARDS_INITIAL_HORIZONTAL_POSITION: f32 = 500.;
pub static DEALER_CARDS_INITIAL_VERTICAL_POSITION: f32 = 100.;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Start,
    InGame,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum DeckState {
    #[default]
    NotShuffled,
    Shuffled
}



