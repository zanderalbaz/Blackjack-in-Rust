pub mod components;
pub mod systems;
pub mod plugins;
pub mod setup;
pub mod constants;
pub mod bundles;

use bevy::prelude::*;
use constants::AppState;
use plugins::StartupPlugin;
use setup::{setupScreen_setup, start_setup};
use systems::{setup_screen, spawn_test_player, start_game, test_player_balance_change};

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
	.add_systems(Update, setupScreen_setup.in_set(SetupGameSystemSet).run_if(in_state(AppState::Setup).and_then(run_once())))
	.add_systems(Update, setup_screen.in_set(SetupGameSystemSet).run_if(in_state(AppState::Setup).and_then(run_once())))
	.add_systems(Startup, spawn_test_player)
	.add_systems(Update, test_player_balance_change)
	.add_plugins(StartupPlugin)
	.insert_state(AppState::Start)
	
	.run();
	println!("Application ran!");
}

