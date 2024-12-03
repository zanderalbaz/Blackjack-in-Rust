pub mod components;
pub mod systems;
pub mod plugins;
pub mod setup;
pub mod constants;

use bevy::prelude::*;
use constants::AppState;
use plugins::SetupPlugin;
use setup::{setupScreen_setup, start_setup};
use systems::start_game;



pub fn run(){
	//start game here.
	App::new()
	.add_systems(Startup, start_setup)
	.add_plugins(SetupPlugin)
	.insert_state(AppState::Start)
	.add_systems(Update, start_game)
    .insert_resource(State::<AppState>::new(AppState::Start))
	.run();
	println!("Application ran!");
}

