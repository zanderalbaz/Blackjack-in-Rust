///components module is used to hold all of our components / structs / enums that we utilize from other modules in the game

use bevy::prelude::*;
use rand::Rng;

use crate::game::traits::{Shufflable, Dealable};

// start screen ---------------

///struct for background image
#[derive(Component)]
pub struct Background;

///struct for logo
#[derive(Component)]
pub struct Logo;

///struct for press enter to play text
#[derive(Component)]
pub struct PressEnterToPlay;

// -----------------------------

// in game screen ---------------

///struct / component for accessing the bet amount text in game
#[derive(Component)]
pub struct BetAmountText {
    pub bet_text: String,
    
}
impl Default for BetAmountText {
    fn default() -> Self {
        BetAmountText {
            bet_text: "0".to_string(),
        }
    }
}

///enum / component used to access and uniquely identify chip buttons 
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum ChipButtonValue {
    One,
    Five,
    Ten,
    Fifty,
}

/// enum / component used to access and uniquely identify specific player system buttons
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum PlayerButtonValues {
    Hit,
    Stand,
    DoubleDown,
    Home,
    Deal,
    KeepPlaying,
}

///enum / component used to access the cards spawned in the screen in game
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum InGameCardAccess {
    DealerCard(usize),
    PlayerCard(usize),
}

/// enum / component used to access and uniquely identify text components in the in game UI
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum TextComponents {
    Balance,
    Bet,
    Instruction,
    NotChanged,
    ResultText,
}
// -----------------------------

// card / deck components --------------

///struct / component used to create and access cards and maintain the data associated with each card
#[derive(Component, Clone)]
pub struct Card{
    pub suite: String,
    pub face: String,
    //Tuple used for implementing aces (1 or 11)
    pub value: (u8,u8), 
    pub front_asset_path: String,
    pub back_asset_path: String,
}

/// struct / resource used to access and utilize the deck in various parts of the game
#[derive(Resource, Component, Clone)]
pub struct Deck{
    pub cards: Vec<Card>,
    pub last_dealt_index: usize
}

//setting up ability for deck to be shuffled
impl Shufflable for Deck {
    fn shuffle(&mut self){
        for i in 0..self.cards.len(){
            //swap current card with random card
            let card1 = self.cards[i].clone();
            let rand_index = rand::thread_rng().gen_range(0..self.cards.len());
            let card2 = self.cards[rand_index].clone();
            self.cards[i] = card2;
            self.cards[rand_index] = card1; 
        }
    }
}

//implementing ability to deal from the deck
impl Dealable for Deck {
    fn deal(&mut self) -> Card{
        if self.last_dealt_index == 51 {
            println!("Shuffling");
            self.shuffle();
            self.last_dealt_index = 0;
            return self.cards[self.last_dealt_index].clone()
        }
        self.last_dealt_index += 1;
        self.cards[self.last_dealt_index].clone()
    }
}

//setting up default values for the cards in the deck
impl Default for Deck {
    fn default() -> Self {
        let mut cards: Vec<Card> = Vec::new();
        //We can probably extract suites and face_values into constants later. Fine for now
        let suites = [
            String::from("spades"),
            String::from("clubs"),
            String::from("diamonds"),
            String::from("hearts")
            ];
        let face_values = [
            (String::from("ace"), (1,11)),
            (String::from("2"), (2,2)),
            (String::from("3"), (3,3)),
            (String::from("4"), (4,4)),
            (String::from("5"), (5,5)),
            (String::from("6"), (6,6)),
            (String::from("7"), (7,7)),
            (String::from("8"), (8,8)),
            (String::from("9"), (9,9)),
            (String::from("10"), (10,10)),
            (String::from("jack"), (10,10)),
            (String::from("queen"), (10,10)),
            (String::from("king"), (10,10)),
            ];

        for suite in &suites{
            for (face, value) in &face_values{
                //dynamically adding the picture for each card 
                let front_asset_path = format!("deck/{}_of_{}.png", face, suite);
                cards.push(
                    Card{ 
                        suite: suite.clone(), 
                        face: face.clone(), 
                    value: *value,
                    front_asset_path: front_asset_path,
                    back_asset_path: String::from("deck/card_back.png") }
                )
            }
        } 
        let mut deck = Self { cards: cards, last_dealt_index: 0};

        deck.shuffle(); 
        deck

    }
}

///struct / component to allow for multiple decks
#[derive(Component)]
pub struct Decks{
    pub number_of_decks: u8,
    pub decks: Vec<Deck>
}

impl Shufflable for Decks {
    fn shuffle(&mut self){
        for deck in &mut self.decks{
            deck.shuffle();
        }
    }
}

impl Dealable for Decks{
    fn deal(&mut self) -> Card {
        let mut deck_to_deal_from = self.decks[rand::thread_rng().gen_range(0..self.number_of_decks) as usize].clone();
        deck_to_deal_from.deal()
    }
}

impl Default for Decks{
    fn default() -> Self {
        Self { number_of_decks: 1, decks: vec![Deck::default()] }
    }
}
// -----------------------------

// player / dealer components ------------

///struct / component for the player name text object in UI
#[derive(Component)]
pub struct PlayerName(pub String);

///struct / component for the player balance text object in UI
#[derive(Component)]
pub struct PlayerBalance(pub f64);

///struct / component for the player hand, holds the current cards for player
#[derive(Component)]
pub struct PlayerHand{
    pub cards: Vec<Card>,
    pub bet: u64
}

///struct / component for multiple player hands 
#[derive(Component)]
pub struct PlayerHands(pub Vec<PlayerHand>);

///struct / component for the dealer hand, holds the current cards for dealer
#[derive(Component)]
pub struct DealerHand{
    pub cards: Vec<Card>
}

// -----------------------------

