use bevy::prelude::*;
use crate::game::components::{Card, Decks, PlayerBalance, PlayerHand, PlayerHands, PlayerName};
use crate::game::bundles::PlayerBundle;

use super::components::Deck;
use super::resources::BalanceValue;
use super::traits::{Dealable, Shufflable};

pub fn initial_shuffle(mut deck: ResMut<Deck>) {
    if deck.cards.is_empty() {
        *deck = Deck::default();
        deck.shuffle();
    }
}

pub fn spawn_test_player(mut commands: Commands, mut deck: ResMut<Deck>){

    if deck.last_dealt_index == 0 {
        deck.shuffle(); 
    }

    let card1 = deck.deal();
    let card2 = deck.deal();

    commands.spawn(PlayerBundle{
        player_name: PlayerName(String::from("test").into()),
        player_balance: PlayerBalance(100.),
        player_hands: PlayerHands(vec![PlayerHand{
            bet: 100,
            cards: vec![card1, card2], 
        }]),
    });
}

pub fn spawn_player(mut commands: Commands, mut deck: ResMut<Deck>, mut balance: ResMut<BalanceValue>){
    if deck.last_dealt_index == 0 {
        deck.shuffle(); 
    }
    let mut cards: Vec<Card> = vec![];
    for i in 0..2{
        cards.push(deck.deal());
    }
    commands.spawn(PlayerBundle{
        player_name: PlayerName(String::from("").into()),
        player_balance: PlayerBalance(balance.value as f64),
        player_hands: PlayerHands(vec![PlayerHand{
            bet: 0,
            cards: cards, 
        }]),
    });
}


pub fn hit_player_hand(mut query: Query<(&mut PlayerHand, &mut PlayerBalance)>){

    println!("hello from hit");
    
    //TODO: figure out how to select the correct hand
    for player_hand in &mut query{
        //add a card to the correct hand
        //check for hand bust (over 21)
    }
}

pub fn stand_player_hand(mut query: Query<(&mut PlayerHand, &mut PlayerBalance)>){

    println!("hello from stand");

    //TODO: figure out how to select the correct hand
    
    //implement some sort of current hand state.
    //end current hand and move to the next hand (dealer or player's next hand)
}

pub fn double_down_player_hand(mut query: Query<(&mut PlayerHand, &mut PlayerBalance)>){
    
    println!("hello from double down");
    //TODO: figure out how to select the correct hand
    
    //check if player has enough balance to double down on this hand
    //yes?
        //update balance
        //double bet for this hand
        //Deal 1 more card to this hand
        //end this hand (switch state?)
    //no?
        //notify player insufficient balance to double down
}

pub fn test_player_hand(mut query: Query<&mut PlayerHands>){
    for player_hand in &mut query{
        let bet = player_hand.0[0].bet;
        let card1 = &player_hand.0[0].cards[0];
        let card2 = &player_hand.0[0].cards[1];
        println!("Bet of {} for cards: {} of {}, {} of {}", bet, card1.face, card1.suite, card2.face, card2.suite);
    }
}

pub fn test_player_balance_change(mut query: Query<&mut PlayerBalance>){
    for mut balance in &mut query{
        println!("Player has balance of {}", balance.0);
        balance.0 += 1.;
        println!("Player has updated balance of {}", balance.0);
    }
}

