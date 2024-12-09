use bevy::prelude::*;
use crate::game::components::{PlayerButtonValues, Card, Decks, PlayerBalance, PlayerHand, PlayerHands, PlayerName};
use crate::game::bundles::PlayerBundle;
use crate::game::constants::{PLAYER_CARDS_INITIAL_HORIZONTAL_POSITION, PLAYER_CARDS_INITIAL_VERTICAL_POSITION, CARD_HORIZONTAL_SPACING, CARD_VERTICAL_SPACING};
use crate::game::in_game_systems::spawn_player_card_after_setup;
use super::components::Deck;
use super::resources::{BalanceValue};
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


pub fn hit_player_hand( 
    mut commands: Commands,
    mut deck: ResMut<Deck>,
    assets: Res<AssetServer>,
    mut player_query: Query<(&mut PlayerHands, &mut PlayerBalance)>,
    mut hit_button_query: Query<(&Button, &mut Interaction, &PlayerButtonValues)>,
){
    // println!("hello from hit");
    
    //TODO: figure out how to select the correct hand
    for (_, mut interaction, value) in hit_button_query.iter_mut(){
        match *interaction{
            Interaction::Pressed => {
                match *value{
                    PlayerButtonValues::Hit => {    
                        *interaction = Interaction::None;
                        let (mut player_hands, _) = player_query.single_mut(); 
                        let player_hand = &mut player_hands.0[0];
                        let insert_index = player_hand.cards.len();
                        let card_to_insert = deck.deal();
                        player_hand.cards.push(card_to_insert.clone());
                        let position = Vec2 {
                            x: PLAYER_CARDS_INITIAL_HORIZONTAL_POSITION + (insert_index as f32)*CARD_HORIZONTAL_SPACING,
                            y: PLAYER_CARDS_INITIAL_VERTICAL_POSITION  + (insert_index as f32)*CARD_VERTICAL_SPACING};
                        
                        println!("Inserting card: {} of {} into player hand at index {}",   player_hand.cards[insert_index].face, player_hand.cards[insert_index].suite, insert_index);
                        //Changing the z-index does not spawn the card on top of the other cards.
                        //Tried:
                        //(1) Editing in_game_setup to not spawn cards as children -- This breaks the add_system command for in_game_setup
                        //(2) Changing the z-index of the in_game_setup cards to be less than the newly spawned card -- This does not fix the issue
                        spawn_player_card_after_setup(
                            &mut commands, 
                            &assets, 
                            &card_to_insert, 
                            insert_index, 
                            position,
                            insert_index as f32 + 100.0,
                        );
                    
                        *interaction = Interaction::None;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    //     //check for hand bust (over 21)
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

