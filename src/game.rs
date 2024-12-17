///game module is the central core of the game where resources, systems, states, and plugins are added.
/// this is also responsible for running specific systems under certain conditions when certain actions occur.

pub mod components;
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
use constants::{AppState, DeckState, GameRoundState};
use in_game_systems::{chip_button_click_system, despawn_cards_and_reset, in_game_setup, player_button_system, print_all_dealer_cards, reset_game, track_app_state, track_game_state};
use resources::{BalanceValue, BetValue, ParentNode};
use setup::{ingame_screen_setup, reload_home_screen, start_setup};
use start_game_systems::start_game;
use plugins::StartupPlugin;

use player_systems::{hit_player_hand, stand_player_hand, double_down_player_hand, initial_shuffle, spawn_player, spawn_test_player};
use dealer_systems::{play_dealer_hand, reveal_dealer_hand, shuffle_dealer_decks, spawn_dealer, spawn_test_dealer};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct StartGameSystemSet;
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SetupGameSystemSet;
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct DeckSystemSet;
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerGameplaySet;

pub fn run() {
    App::new()
        // Set system execution order
        .configure_sets(Startup, StartGameSystemSet.before(SetupGameSystemSet))
        .configure_sets(Startup, SetupGameSystemSet.before(DeckSystemSet))
        .configure_sets(Update, PlayerGameplaySet.run_if(in_state(AppState::InGame)).run_if(in_state(GameRoundState::PlayerHand)))
        
        // Add startup systems
        .add_systems(Startup, start_setup.in_set(StartGameSystemSet))
        .add_systems(Startup, initial_shuffle)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_dealer)
        
        // Add resources
        .insert_resource(BetValue { value: 0 })
        .insert_resource(BalanceValue { value: 1000 })
        .insert_resource(Deck::default())
        .insert_resource(ParentNode::default())

        // Add in-game systems
        .add_systems(Update, start_game.in_set(StartGameSystemSet))
        .add_systems(Update, ingame_screen_setup.in_set(SetupGameSystemSet).run_if(in_state(AppState::InGame).and_then(run_once())))
        .add_systems(Update, in_game_setup.in_set(SetupGameSystemSet).run_if(in_state(AppState::InGame).and_then(run_once())))
        .add_systems(Update, chip_button_click_system.in_set(SetupGameSystemSet).run_if(in_state(AppState::InGame)))
        .add_systems(Update, player_button_system.in_set(SetupGameSystemSet).run_if(in_state(AppState::InGame)))
        
        // Add player gameplay systems
        .add_systems(Update, hit_player_hand.in_set(PlayerGameplaySet).run_if(in_state(AppState::InGame)))
        .add_systems(Update, stand_player_hand.in_set(PlayerGameplaySet).run_if(in_state(AppState::InGame)))
        .add_systems(Update, double_down_player_hand.in_set(PlayerGameplaySet).run_if(in_state(AppState::InGame)))

        // Add dealer systems
        .add_systems(OnEnter(GameRoundState::DealerHand), play_dealer_hand)
        
        // Track game states on entry
        .add_systems(OnEnter(GameRoundState::PlayerHand), track_game_state)
        .add_systems(OnEnter(GameRoundState::DealerHand), track_game_state)
        .add_systems(OnEnter(GameRoundState::RoundEnd), track_game_state)
        .add_systems(OnEnter(GameRoundState::RoundStart), track_game_state)
        .add_systems(OnEnter(GameRoundState::Betting), track_game_state)

        .add_systems(OnEnter(GameRoundState::Betting), despawn_cards_and_reset)
        .add_systems(OnEnter(GameRoundState::Betting), in_game_setup)
        
        // Track app states on entry
        .add_systems(OnEnter(AppState::Start), track_app_state)
        .add_systems(OnEnter(AppState::InGame), track_app_state)

        .add_systems(OnEnter(AppState::Start), reset_game)

        // Clean-up on exit
        .add_systems(OnExit(AppState::InGame), reload_home_screen)
        .add_systems(OnExit(AppState::Start), ingame_screen_setup)
        .add_systems(OnExit(AppState::Start), in_game_setup)
        .add_systems(OnExit(GameRoundState::PlayerHand), reveal_dealer_hand)

        // Add plugins and set initial states
        .add_plugins(StartupPlugin)
        .insert_state(AppState::Start)
        .insert_state(DeckState::NotShuffled)
        .insert_state(GameRoundState::RoundStart)
        
        .run();
}


//testing systems
	// .add_systems(Startup, spawn_test_player)
	// .add_systems(Update, test_player_balance_change.in_set(SetupGameSystemSet).run_if(in_state(AppState::InGame).and_then(run_once())))
	// .add_systems(Update, test_player_hand.in_set(SetupGameSystemSet).run_if(in_state(AppState::InGame).and_then(run_once())))

	// .add_systems(Startup, spawn_test_dealer)
	// .add_systems(Update, shuffle_dealer_decks.in_set(DeckSystemSet).run_if(in_state(DeckState::NotShuffled).and_then(run_once())))
	// .add_systems(Update, test_dealer_decks.in_set(DeckSystemSet).run_if(in_state(DeckState::Shuffled).and_then(run_once())))
	// .add_systems(Update, test_dealer_hand.in_set(DeckSystemSet).run_if(in_state(DeckState::Shuffled).and_then(run_once())))