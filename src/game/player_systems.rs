use bevy::prelude::*;
use crate::game::components::{PlayerName, PlayerBalance, PlayerHand, PlayerHands, Card};
use crate::game::bundles::PlayerBundle;

pub fn spawn_test_player(mut commands: Commands){
    commands.spawn(PlayerBundle{
        player_name: PlayerName(String::from("test").into()),
        player_balance: PlayerBalance(100.),
        player_hands: PlayerHands(vec![PlayerHand{
            bet: 100,
            cards: //This is good for a test, but later we should spawn card components
            vec![Card{ 
                suite: String::from("spades").into(), 
                face: String::from("2").into(), 
                value: (3,0),
                front_asset_path: String::from("deck/2_of_spades.png").into(),
                back_asset_path: String::from("deck/card_back.png").into()
            }, 
            Card{
                suite: String::from("spades").into(),
                face: String::from("3").into(),
                value: (3,0),
                front_asset_path: String::from("deck/3_of_spades.png").into(),
                back_asset_path: String::from("deck/card_back.png").into()
            }]
        }]),
    });
}





pub fn hit_player_hand(mut query: Query<&mut PlayerHands>){
    //TODO: figure out how to select the correct hand
    for player_hand in &mut query{
        //add a card to the correct hand
        //check for hand bust (over 21)
    }
}

pub fn stand_player_hand(mut query: Query<&mut PlayerHands>){
    //TODO: figure out how to select the correct hand
    
    //implement some sort of current hand state.
    //end current hand and move to the next hand (dealer or player's next hand)
}

pub fn double_down_player_hand(mut query: Query<(&mut PlayerHands, &mut PlayerBalance)>){
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

pub fn split_player_hand(mut query: Query<(&mut PlayerHands, &mut PlayerBalance)>){
    //TODO: figure out how to select the correct hand
    
    //check if player has enough balance to split this hand
    //check if this hand has correct split conditions (2 of the same face)      << We might be able to extract this functionality into the Split button
        //yes?
            //spawn new hand (with a card from previous card)
            //remove said card from current hand
            //Set some state indicating that the player has N hands
            //resume play for the first hand
        //no?
            //return play as normal but notify player that they cannot split their hand (insufficient funds or invalid hand)
}


//TODO: figure out how to check when a chip denomination is clicked
pub fn bet_player_hand(mut query: Query<(&mut PlayerHands, &mut PlayerBalance)>){
    //TODO: figure out how to select the correct hand

    //check if player has enough balance to bet their selected chip denomination
        //yes?
            //add chip amount to the the bet on the player's hand (I beleive this should only apply to the first hand (i.e. no logic needed for split / double down))
        //no?
            //notify player insufficient funds
}

//TODO: Figure out if we want to implement the surrender feature for hands (leaning on no)




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

