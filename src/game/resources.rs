use bevy::prelude::*;

#[derive(Resource,Default)]
pub struct BalanceValue {
    pub value: i32,
}

#[derive(Resource,Default)]
pub struct BetValue {
    pub value: i32,
}

#[derive(Resource)]
pub struct ParentNode(pub Entity);


impl Default for ParentNode {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}