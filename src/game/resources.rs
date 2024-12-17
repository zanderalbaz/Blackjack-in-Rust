///resources module holds our resources that we access and update throughout ingame sessions

use bevy::prelude::*;

///struct / resource BalanceValue is used for displaying and updating the player's balance throughout the lifetime of a match
#[derive(Resource,Default)]
pub struct BalanceValue {
    pub value: i32,
}

///struct / resource BetValue is used for displaying and updating the player's bet value throughout the lifetime of a match
#[derive(Resource,Default)]
pub struct BetValue {
    pub value: i32,
}

///struct / resource ParentNode is used to help us access the original parent node to spawn UI elements outside of in_game_systems module
#[derive(Resource)]
pub struct ParentNode(pub Entity);


impl Default for ParentNode {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}