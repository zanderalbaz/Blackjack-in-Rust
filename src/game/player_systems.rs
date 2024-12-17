///player_systems module holds and implements the logic and functionality for the player

use bevy::prelude::*;
use crate::game::components::{PlayerButtonValues, Card, PlayerBalance, PlayerHand, PlayerHands, PlayerName};
use crate::game::bundles::PlayerBundle;
use crate::game::constants::{CARD_HORIZONTAL_SPACING, CARD_VERTICAL_SPACING, NO_CARD_VALUE, PLAYER_CARDS_INITIAL_HORIZONTAL_POSITION, PLAYER_CARDS_INITIAL_VERTICAL_POSITION};
use crate::game::in_game_systems::{spawn_player_card};
use super::components::{DealerHand, Deck, TextComponents};
use super::constants::GameRoundState;
use super::in_game_systems::{spawn_keep_playing_button, spawn_result_text};
use super::resources::{BalanceValue, BetValue, ParentNode};
use super::traits::{Dealable, Shufflable};

///initial_shuffle ensures the deck is shuffled before dealing
pub fn initial_shuffle(mut deck: ResMut<Deck>) {
    if deck.cards.is_empty() {
        *deck = Deck::default();
        deck.shuffle();
    }
}

///spawn_player deals the player a hand and sets up the player to begin the round.
pub fn spawn_player(mut commands: Commands, mut deck: ResMut<Deck>, balance: ResMut<BalanceValue>){
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

///hit_player_hand implements the logic for when the hit button is pressed by the player
pub fn hit_player_hand( 
    mut commands: Commands,
    mut deck: ResMut<Deck>,
    assets: Res<AssetServer>,
    parent_node: Res<ParentNode>,
    mut next_state: ResMut<NextState<GameRoundState>>,
    mut player_query: Query<(&mut PlayerHands, &mut PlayerBalance)>,
    mut hit_button_query: Query<(&Button, &mut Interaction, &PlayerButtonValues)>,
    mut balance: ResMut<BalanceValue>,
    mut bet_amount: ResMut<BetValue>
){    
    for (_, mut interaction, value) in hit_button_query.iter_mut(){
        match *interaction{
            Interaction::Pressed => {
                match *value{
                    PlayerButtonValues::Hit => {    
                        let (mut player_hands, _) = player_query.single_mut(); 
                        let player_hand = &mut player_hands.0[0];
                        let insert_index = player_hand.cards.len();
                        let card_to_insert = deck.deal();
                        player_hand.cards.push(card_to_insert.clone());
                        let position = Vec2 {
                            x: PLAYER_CARDS_INITIAL_HORIZONTAL_POSITION + (insert_index as f32)*CARD_HORIZONTAL_SPACING,
                            y: PLAYER_CARDS_INITIAL_VERTICAL_POSITION  + (insert_index as f32)*CARD_VERTICAL_SPACING};
                        
                        // println!("Inserting card: {} of {} into player hand at index {}",   player_hand.cards[insert_index].face, player_hand.cards[insert_index].suite, insert_index);
                        
                        commands.entity(parent_node.0).with_children(|parent|{
                            spawn_player_card(
                                parent,
                                &assets, 
                                &card_to_insert, 
                                insert_index, 
                                position,
                                true,
                            );
                        });
                        let bust = determine_player_bust(player_hand);
                        if bust{

                            commands.entity(parent_node.0).with_children(|parent|{
                                let result = format!("You Lose ${}! (Bust)", bet_amount.value);
                                spawn_result_text(
                                    parent,
                                    &assets,
                                    &result
                                );

                                spawn_keep_playing_button(parent, &assets);
                            });

                            bet_amount.value = 0;
                            next_state.set(GameRoundState::RoundEnd);
                            
                        }
                        *interaction = Interaction::None;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

}

///stand_player_hand is used to implement the logic for when the player presses the stand button.
pub fn stand_player_hand(
    mut next_state: ResMut<NextState<GameRoundState>>,
    mut stand_button_query: Query<(&Button, &mut Interaction, &PlayerButtonValues)>,
){

    for (_, mut interaction, value) in stand_button_query.iter_mut(){
        match *interaction{
            Interaction::Pressed => {
                match *value{
                    PlayerButtonValues::Stand => {    
                        next_state.set(GameRoundState::DealerHand);
                        *interaction = Interaction::None;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }    
}

///double_down_player_hand is responsible for implementing the logic associated with when the player presses double down.
pub fn double_down_player_hand(
    mut commands: Commands,
    mut bet_value: ResMut<BetValue>,
    mut balance_value: ResMut<BalanceValue>,
    mut deck: ResMut<Deck>,
    assets: Res<AssetServer>,
    parent_node: Res<ParentNode>,
    mut next_state: ResMut<NextState<GameRoundState>>,
    mut text_query: Query<(&TextComponents, &mut Text)>,
    mut player_query: Query<(&mut PlayerHands, &mut PlayerBalance)>,
    mut double_down_button_query: Query<(&Button, &mut Interaction, &PlayerButtonValues)>,
){
    for (_, mut interaction, value) in double_down_button_query.iter_mut(){
        match *interaction{
            Interaction::Pressed => {
                match *value{
                    PlayerButtonValues::DoubleDown => {    
                        //Check player balance:
                        let balance = balance_value.value;
                        let bet = bet_value.value;
                        if balance < bet {
                            *interaction = Interaction::None;
                            println!("Insufficient balance to double down");
                            return;
                        }
                        else {
                            let new_bet_text = (bet*2).to_string();
                            let new_balance_text = (balance-bet).to_string();
                            balance_value.value -= bet_value.value;
                            bet_value.value = bet_value.value*2;
                            for (text_component, mut text) in text_query.iter_mut() {
                                if let TextComponents::Bet = text_component {
                                    text.sections[0].value = new_bet_text.clone(); 
                                }
                    
                                if let TextComponents::Balance = text_component {
                                    text.sections[0].value = new_balance_text.clone();
                                }
                            }
                    
                        }
                        //Player balance is valid
                        let (mut player_hands, _) = player_query.single_mut(); 
                        let player_hand = &mut player_hands.0[0];
                        let insert_index = player_hand.cards.len();
                        let card_to_insert = deck.deal();
                        player_hand.cards.push(card_to_insert.clone());
                        let position = Vec2 {
                            x: PLAYER_CARDS_INITIAL_HORIZONTAL_POSITION + (insert_index as f32)*CARD_HORIZONTAL_SPACING,
                            y: PLAYER_CARDS_INITIAL_VERTICAL_POSITION  + (insert_index as f32)*CARD_VERTICAL_SPACING};
                        commands.entity(parent_node.0).with_children(|parent|{
                            spawn_player_card(
                                parent,
                                &assets, 
                                &card_to_insert, 
                                insert_index, 
                                position,
                                true,
                            );
                        });
                        
                        let bust = determine_player_bust(player_hand);
                        if bust{
                            next_state.set(GameRoundState::RoundEnd);
                        }
                        else {
                            next_state.set(GameRoundState::DealerHand);
                        }
                        *interaction = Interaction::None;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }    
    //check if player has enough balance to double down on this hand
    //yes?
        //update balance
        //double bet for this hand
        //Deal 1 more card to this hand
        //end this hand (switch state?)
    //no?
        //notify player insufficient balance to double down
}

///determine_player_bust is used for implementing the logic when a player's hand is a bust.
pub fn determine_player_bust(player_hand: &mut PlayerHand)-> bool{
    let mut totals: (u8, u8) = (0,0);
    for card in &player_hand.cards{
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

///spawn_test_player is used to run a test player in the command line to monitor the values associated with a player
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

///test_player_hand is used for testing and monitoring the values of a player's hand in the command line
pub fn test_player_hand(mut query: Query<&mut PlayerHands>){
    for player_hand in &mut query{
        let bet = player_hand.0[0].bet;
        let card1 = &player_hand.0[0].cards[0];
        let card2 = &player_hand.0[0].cards[1];
        println!("Bet of {} for cards: {} of {}, {} of {}", bet, card1.face, card1.suite, card2.face, card2.suite);
    }
}

///test_player_balance_change is used for testing and monitoring the player's balance when a change is supposed to occur.
pub fn test_player_balance_change(mut query: Query<&mut PlayerBalance>){
    for mut balance in &mut query{
        println!("Player has balance of {}", balance.0);
        balance.0 += 1.;
        println!("Player has updated balance of {}", balance.0);
    }
}
