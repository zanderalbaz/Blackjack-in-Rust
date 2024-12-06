pub mod components;
pub mod systems;
pub mod player_systems;
pub mod dealer_systems;
pub mod plugins;
pub mod setup;
pub mod constants;
pub mod bundles;
pub mod start_game_systems;
pub mod in_game_systems;
pub mod traits;
pub mod resources;

use bevy::prelude::*;
use components::Deck;
use constants::{AppState, DeckState};

use in_game_systems::{ chip_button_click_system, player_button_system, inGame_setup};
use resources::{BalanceValue, BetValue};
use start_game_systems::start_game;
use plugins::StartupPlugin;
use setup::{setupScreen_setup, start_setup};
use player_systems::{hit_player_hand, initial_shuffle, spawn_test_player, test_player_balance_change, test_player_hand};
use dealer_systems::{shuffle_dealer_decks, spawn_test_dealer, test_dealer_decks, test_dealer_hand};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct StartGameSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SetupGameSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct DeckSystemSet;

pub fn run(){
	//start game here.
	App::new()
	//startup
	.configure_sets(Startup, StartGameSystemSet.before(SetupGameSystemSet))
	.configure_sets(Startup, SetupGameSystemSet.before(DeckSystemSet))
	.add_systems(Startup, start_setup.in_set(StartGameSystemSet))
	.add_systems(Startup, initial_shuffle)

	//resources
	.insert_resource(BetValue { value: 0 })
	.insert_resource(BalanceValue { value: 1000 })
	.insert_resource(Deck::default())

	//ingame
	.add_systems(Update, start_game.in_set(StartGameSystemSet))
	.add_systems(Update, setupScreen_setup.in_set(SetupGameSystemSet).run_if(in_state(AppState::InGame).and_then(run_once())))
	.add_systems(Update, inGame_setup.in_set(SetupGameSystemSet).run_if(in_state(AppState::InGame).and_then(run_once())))
	.add_systems(Update, chip_button_click_system.in_set(SetupGameSystemSet).run_if(in_state(AppState::InGame)))
	.add_systems(Update, player_button_system.in_set(SetupGameSystemSet).run_if(in_state(AppState::InGame)))

	//testing systems
	.add_systems(Startup, spawn_test_player)
	.add_systems(Update, test_player_balance_change.in_set(SetupGameSystemSet).run_if(in_state(AppState::InGame).and_then(run_once())))
	.add_systems(Update, test_player_hand.in_set(SetupGameSystemSet).run_if(in_state(AppState::InGame).and_then(run_once())))

	.add_systems(Startup, spawn_test_dealer)
	.add_systems(Update, shuffle_dealer_decks.in_set(DeckSystemSet).run_if(in_state(DeckState::NotShuffled).and_then(run_once())))
	.add_systems(Update, test_dealer_decks.in_set(DeckSystemSet).run_if(in_state(DeckState::Shuffled).and_then(run_once())))
	.add_systems(Update, test_dealer_hand.in_set(DeckSystemSet).run_if(in_state(DeckState::Shuffled).and_then(run_once())))

	.add_plugins(StartupPlugin)
	.insert_state(AppState::Start)
	.insert_state(DeckState::NotShuffled)
	
	.run();
	println!("Application ran!");
}

