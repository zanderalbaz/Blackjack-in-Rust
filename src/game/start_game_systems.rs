use bevy::prelude::*;
use super::constants::AppState;

pub fn start_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,

){
    if keyboard_input.just_pressed(KeyCode::Enter) {

        next_state.set(AppState::InGame);

    }
}
