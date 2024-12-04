use bevy::prelude::*;

use super::constants;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Logo;

#[derive(Component)]
pub struct PressEnterToPlay;


#[derive(Component)]
pub struct Card{
    pub suite: String,
    pub face: String,
    //Tuple used for implementing aces (1 or 11)
    pub value: (u8,u8), 
    pub front_asset_path: String,
    pub back_asset_path: String
}

#[derive(Component)]
pub struct Deck{
    pub cards: Vec<Card>
}

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
        let face_values: [(String, (u8, u8)); 13] = [
            (String::from("ace"), (1,11)),
            (String::from("2"), (2,constants::NO_CARD_VALUE)),
            (String::from("3"), (3,constants::NO_CARD_VALUE)),
            (String::from("4"), (4,constants::NO_CARD_VALUE)),
            (String::from("5"), (5,constants::NO_CARD_VALUE)),
            (String::from("6"), (6,constants::NO_CARD_VALUE)),
            (String::from("7"), (7,constants::NO_CARD_VALUE)),
            (String::from("8"), (8,constants::NO_CARD_VALUE)),
            (String::from("9"), (9,constants::NO_CARD_VALUE)),
            (String::from("10"), (10,constants::NO_CARD_VALUE)),
            (String::from("jack"), (10,constants::NO_CARD_VALUE)),
            (String::from("queen"), (10,constants::NO_CARD_VALUE)),
            (String::from("king"), (10,constants::NO_CARD_VALUE)),
            ];

        for suite in &suites{
            for (face, value) in &face_values{
                let front_asset_path = format!("deck/{}_of_{}.png", face, suite);
                cards.push(Card { suite: suite.clone(), face: face.clone(), value: *value, front_asset_path: front_asset_path, back_asset_path: String::from("deck/card_back.png") })
            }
        } 
        Self { cards: cards}
    }
}


#[derive(Component)]
pub struct Decks{
    pub number_of_decks: u8,
    pub decks: Vec<Deck>
}

impl Default for Decks{
    fn default() -> Self {
        Self { number_of_decks: 1, decks: vec![Deck::default()] }
    }
}


#[derive(Component)]
pub struct PlayerName(pub String);

#[derive(Component)]
pub struct PlayerBalance(pub f64);

#[derive(Component)]
pub struct PlayerHand{
    pub cards: Vec<Card>
}


//Need this Hands component in order to create a valid PlayerBundle
#[derive(Component)]
pub struct PlayerHands(pub Vec<PlayerHand>);


#[derive(Component)]
pub struct DealerHand{
    pub cards: Vec<Card>
}

