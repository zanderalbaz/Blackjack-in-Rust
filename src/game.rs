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

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct StartGameSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SetupGameSystemSet;


pub fn run(){
	//start game here.
	App::new()
	.configure_sets(Startup, StartGameSystemSet.before(SetupGameSystemSet))
	.add_systems(Startup, start_setup.in_set(StartGameSystemSet))
	.add_systems(Update, start_game.in_set(StartGameSystemSet))
	.add_systems(Update, setupScreen_setup.in_set(SetupGameSystemSet).run_if(in_state(AppState::Setup)))
	.add_plugins(SetupPlugin)
	.insert_state(AppState::Start)
	
	.run();
	println!("Application ran!");
}

