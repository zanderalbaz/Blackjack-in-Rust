use bevy::prelude::*;
use crate::game::components::{PlayerHands, PlayerName, PlayerBalance, DealerHand, Decks};

#[derive(Bundle)]
pub struct PlayerBundle{
    pub player_name: PlayerName,
    pub player_balance: PlayerBalance,
    pub player_hands: PlayerHands
}

#[derive(Bundle)]
pub struct DealerBundle{
    pub dealer_decks: Decks,
    pub dealer_hand: DealerHand,
}

