use bevy::prelude::*;
use crate::game::components::{Decks, DealerHand, Card, PlayerHands};
use crate::game::bundles::DealerBundle;
use crate::game::constants::DeckState;

use super::components::Deck;
use super::traits::{Dealable, Shufflable};

pub fn spawn_dealer(mut commands: Commands, mut deck: ResMut<Deck>){
    if deck.last_dealt_index == 0 {
        deck.shuffle();
    }

    let dealer_card1 = deck.deal();
    let dealer_card2 = deck.deal();

    let dealer_hand = DealerHand {
        cards: vec![dealer_card1.clone(), dealer_card2.clone()],
    };

    let dealer_decks = Decks {
        number_of_decks: 2,
        decks: vec![Deck::default(), Deck::default()], 
    };

    commands.spawn(DealerBundle {
        dealer_hand,
        dealer_decks,
    });
}

pub fn spawn_test_dealer(mut commands: Commands, mut deck: ResMut<Deck>){
    if deck.last_dealt_index == 0 {
        deck.shuffle();
    }

    let dealer_card1 = deck.deal();
    let dealer_card2 = deck.deal();

    let dealer_hand = DealerHand {
        cards: vec![dealer_card1.clone(), dealer_card2.clone()],
    };

    let dealer_decks = Decks {
        number_of_decks: 2,
        decks: vec![Deck::default(), Deck::default()], 
    };

    commands.spawn(DealerBundle {
        dealer_hand,
        dealer_decks,
    });

    println!("Dealer's hand: {} of {}, {} of {}", dealer_card1.face, dealer_card1.suite, dealer_card2.face, dealer_card2.suite);
}

pub fn test_dealer_decks(mut query: Query<&mut Decks>){
    for deck in &mut query{
        let num_decks = deck.number_of_decks;
        println!("Number of Decks: {num_decks}");
        let deck1 = &deck.decks[0];
        for card in &deck1.cards{
            println!("Card: {} of {}", card.face, card.suite);
        }
    }
}

pub fn test_dealer_hand(mut query: Query<&mut DealerHand>){
    for dealer_hand in &mut query{
        let card1 = &dealer_hand.cards[0];
        let card2 = &dealer_hand.cards[1];
        println!("Dealer Cards: {} of {}, {} of {}", card1.face, card1.suite, card2.face, card2.suite);
    }
}

pub fn shuffle_dealer_decks(mut query: Query<&mut Decks>,
    mut state: ResMut<State<DeckState>>,
    mut next_state: ResMut<NextState<DeckState>>){
   
    for mut decks in &mut query{
        decks.shuffle();
    }
    next_state.set(DeckState::Shuffled)
}

pub fn hit_dealer_hand(mut query: Query<&mut DealerHand>){
    for dealer_hand in &mut query{
        //add a card to dealer hand
        //check for hand bust (over 21)
    }
}

pub fn stand_dealer_hand(mut query: Query<(&mut DealerHand, &mut PlayerHands)>){    
    //end current hand and calculate win
}