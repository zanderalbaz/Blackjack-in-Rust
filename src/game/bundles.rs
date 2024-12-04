use bevy::prelude::*;
use crate::game::components::{Hands, PlayerName, Balance};

#[derive(Bundle)]
pub struct PlayerBundle{
    pub player_name: PlayerName,
    pub balance: Balance,
    pub hands: Hands
}

