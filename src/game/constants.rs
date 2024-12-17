///constants module holds our Constants to be accessed and utilized in all modules

use bevy::prelude::*;

pub static NO_CARD_VALUE: u8 = 0;

pub static CARD_HORIZONTAL_SPACING: f32 = 15.;
pub static CARD_VERTICAL_SPACING: f32 = 15.;
pub static DEALER_CARDS_INITIAL_HORIZONTAL_POSITION: f32 = 400.;
pub static DEALER_CARDS_INITIAL_VERTICAL_POSITION: f32 = 50.;
pub static PLAYER_CARDS_INITIAL_HORIZONTAL_POSITION: f32 = 10.;
pub static PLAYER_CARDS_INITIAL_VERTICAL_POSITION: f32 = 50.;

///enum / States AppState used to track whether the game is in the Start state or InGame  state.
/// also used to transition / set up UI elements based on certain actions (button presses)
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Start,
    InGame,
}

///enum / States DeckState used to track whether the deck is shuffled or not
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum DeckState {
    #[default]
    NotShuffled,
    Shuffled
}

///enum / States GameRoundState is used to track game rounds, turns, and initiate systems / transitions based on actions in game
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameRoundState {
    #[default]
    RoundStart,
    Betting,
    PlayerHand,
    DealerHand,
    RoundEnd
}