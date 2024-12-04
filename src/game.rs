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
use systems::{inGame_setup, spawn_test_player, start_game, test_player_balance_change, test_player_hand, spawn_test_dealer, test_dealer_decks, test_dealer_hand};

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
	.add_systems(Update, setupScreen_setup.in_set(SetupGameSystemSet).run_if(in_state(AppState::InGame).and_then(run_once())))
	.add_systems(Update, inGame_setup.in_set(SetupGameSystemSet).run_if(in_state(AppState::InGame).and_then(run_once())))

	//testing systems
	.add_systems(Startup, spawn_test_player)
	.add_systems(Update, test_player_balance_change.in_set(SetupGameSystemSet).run_if(in_state(AppState::InGame).and_then(run_once())))
	.add_systems(Update, test_player_hand.in_set(SetupGameSystemSet).run_if(in_state(AppState::InGame).and_then(run_once())))

	.add_systems(Startup, spawn_test_dealer)
	.add_systems(Update, test_dealer_decks.in_set(SetupGameSystemSet).run_if(in_state(AppState::InGame).and_then(run_once())))
	.add_systems(Update, test_dealer_hand.in_set(SetupGameSystemSet).run_if(in_state(AppState::InGame).and_then(run_once())))

	
	.add_plugins(StartupPlugin)
	.insert_state(AppState::Start)
	
	.run();
	println!("Application ran!");
}

