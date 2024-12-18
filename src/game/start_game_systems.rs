///start game systems is used for the logic that brings you from the start screen to the in game screen

use bevy::prelude::*;
use super::constants::AppState;

///start_game is used to bring you from the start screen to the in_game screen by having the enter button hit
pub fn start_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,

){
    if keyboard_input.just_pressed(KeyCode::Enter) {

        next_state.set(AppState::InGame);

    }
}
