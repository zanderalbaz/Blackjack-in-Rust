use bevy::prelude::*;
use crate::game::components::{Decks, DealerHand, Card, PlayerHands};
use crate::game::bundles::DealerBundle;

pub fn spawn_test_dealer(mut commands: Commands){
    commands.spawn(DealerBundle{
        dealer_decks: Decks::default(),
        dealer_hand: DealerHand{ 
            cards: vec![Card{ 
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
        },
    });
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




pub fn hit_dealer_hand(mut query: Query<&mut DealerHand>){
    for dealer_hand in &mut query{
        //add a card to dealer hand
        //check for hand bust (over 21)
    }
}

pub fn stand_dealer_hand(mut query: Query<(&mut DealerHand, &mut PlayerHands)>){    
    //end current hand and calculate win

}


pub fn shuffle_dealer_decks(mut query: Query<&mut Decks>){
    //shuffle dealer hand once out of cards.
}

