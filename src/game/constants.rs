use bevy::prelude::*;

pub static NO_CARD_VALUE: u8 = 0;

pub static CARD_HORIZONTAL_SPACING: f32 = 15.;
pub static CARD_VERTICAL_SPACING: f32 = 15.;
pub static DEALER_CARDS_INITIAL_HORIZONTAL_POSITION: f32 = 400.;
pub static DEALER_CARDS_INITIAL_VERTICAL_POSITION: f32 = 50.;
pub static PLAYER_CARDS_INITIAL_HORIZONTAL_POSITION: f32 = 10.;
pub static PLAYER_CARDS_INITIAL_VERTICAL_POSITION: f32 = 50.;

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

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameRoundState {
    #[default]
    RoundStart,
    WaitingForBet,
    Betting,
    Dealing,
    WaitingForPlayerAction,
    PlayerAction,
    DealerAction,
    RoundEnd
}