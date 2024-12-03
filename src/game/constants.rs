use bevy::prelude::*;

#[derive(States)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AppState {
    Start,
    Setup,
    InGame,
}