use bevy::prelude::*;
use crate::game::components::{Decks, DealerHand, Card, PlayerHands};
use crate::game::bundles::DealerBundle;
use crate::game::constants::{DeckState, NO_CARD_VALUE};
use crate::game::in_game_systems::spawn_result_text;

use super::components::{Deck, InGameCardAccess, PlayerBalance, PlayerHand};
use super::constants::{GameRoundState, CARD_HORIZONTAL_SPACING, CARD_VERTICAL_SPACING, DEALER_CARDS_INITIAL_HORIZONTAL_POSITION, DEALER_CARDS_INITIAL_VERTICAL_POSITION};
use super::in_game_systems::spawn_dealer_card;
use super::resources::{BalanceValue, BetValue, ParentNode};
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

pub fn reveal_dealer_hand(
    mut dealer_card_image_query: Query<(&InGameCardAccess, &mut UiImage)>,
    dealer_hand_query: Query<&DealerHand>,
    assets: Res<AssetServer>,
){
    for (card_access, mut ui_image) in dealer_card_image_query.iter_mut() {
        if let InGameCardAccess::DealerCard(card_index) = card_access {
            if *card_index == 0 {
                for dealer_hand in dealer_hand_query.iter(){
                    let card = &dealer_hand.cards[0];
                    let card_front_texture = assets.load(card.front_asset_path.clone());
                    ui_image.texture = card_front_texture;
                }
                }
              
        }
    }
}

pub fn play_dealer_hand(
    mut commands: Commands,
    mut deck: ResMut<Deck>,
    assets: Res<AssetServer>, 
    parent_node: Res<ParentNode>,
    mut next_state: ResMut<NextState<GameRoundState>>,
    mut query: Query<&mut DealerHand>,
    mut balance: ResMut<BalanceValue>,
    mut bet_amount: ResMut<BetValue>

){
    
    for mut dealer_hand in &mut query{
        let mut totals: (u8, u8) = (0,0);
    for card in &dealer_hand.cards{
        let (card_total1, card_total2) = card.value;
        totals.0 += card_total1;
        totals.1 += card_total2;
    }
        //Hit on soft 17
        while totals.0 < 17 || totals.1 < 18 {
            let insert_index = dealer_hand.cards.len();
            let card_to_insert = deck.deal();
            dealer_hand.cards.push(card_to_insert.clone());
            let position = Vec2 {
                x: DEALER_CARDS_INITIAL_HORIZONTAL_POSITION + (insert_index as f32)*CARD_HORIZONTAL_SPACING,
                y: DEALER_CARDS_INITIAL_VERTICAL_POSITION  + (insert_index as f32)*CARD_VERTICAL_SPACING};
                commands.entity(parent_node.0).with_children(|parent|{
                    spawn_dealer_card(
                        parent,
                        &assets, 
                        &card_to_insert, 
                        insert_index, 
                        position,
                        true,
                        true
                    );
                });
            let (card_value1, card_value2) = card_to_insert.value;
            totals.0 += card_value1;
            totals.1 += card_value2;

            //Maybe add a small delay here
        }

        if determine_dealer_bust(&mut dealer_hand) {
            println!("Dealer Bust: Player Wins!");

            commands.entity(parent_node.0).with_children(|parent|{
                let result = format!("You Win ${}! (Dealer Bust)", bet_amount.value);
                spawn_result_text(
                    parent,
                    &assets,
                    &result
                );
            });

            balance.value += bet_amount.value * 2;
            bet_amount.value = 0;

            next_state.set(GameRoundState::RoundEnd);  
            return; 
        }

        next_state.set(GameRoundState::RoundEnd);

    }
}

pub fn determine_dealer_bust(dealer_hand: &mut DealerHand)-> bool{
    let mut totals: (u8, u8) = (0,0);
    for card in &dealer_hand.cards{
        let (card_total1, card_total2) = card.value;
        totals.0 += card_total1;
        totals.1 += card_total2;
    }
    if totals.1 > 21 && totals.0 > 21 {
        return true;
    }
    else{
        return false;
    }

}

pub fn determine_win() {

    // Maybe replace bust function with this ?

    // Retrieve the player's hand
    
    // Retrieve the dealer's hand
    
    // Calculate hand totals for both player and dealer into player_total and dealer_total

    /*
        Determine the outcome of the round
        
        if player_total > 21 {
            // Player busts, dealer wins
            println!("Player busts! Dealer wins.");
            adjust_balance(&mut player_balance, -bet_value.value);
            next_state.set(GameRoundState::RoundEnd);
        } else if dealer_total > 21 {
            // Dealer busts, player wins
            println!("Dealer busts! Player wins.");
            adjust_balance(&mut player_balance, bet_value.value);
            next_state.set(GameRoundState::RoundEnd);
        } else {
            // No one busted, compare totals
            if player_total > dealer_total {
                // Player wins
                println!("Player wins!");
                adjust_balance(&mut player_balance, bet_value.value);
                next_state.set(GameRoundState::RoundEnd);
            } else if dealer_total > player_total {
                // Dealer wins
                println!("Dealer wins!");
                adjust_balance(&mut player_balance, -bet_value.value);
                next_state.set(GameRoundState::RoundEnd);
            } else {
                // It's a tie (push)
                println!("It's a tie! Push.");
                next_state.set(GameRoundState::RoundEnd);
            }
        }
     */
    
}


// for player_hand in &mut player_query {
//     let mut player_totals: (u8, u8) = (0,0);

//     for card in &player_hand.cards {
//         let (card_total1, card_total2) = card.value;
//         player_totals.0 += card_total1;
//         player_totals.1 += card_total2;

//     }
//     let player_total = player_totals.0 + player_totals.1;
//     println!("{player_total}");
// }

// let dealer_total = totals.0 + totals.1;
// println!("{dealer_total}");