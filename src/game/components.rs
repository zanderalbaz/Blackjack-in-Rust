use bevy::prelude::*;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Logo;

#[derive(Component)]
pub struct PressEnterToPlay;

#[derive(Component)]
pub struct PlayerName(pub String);

#[derive(Component)]
pub struct Balance(pub f64);

#[derive(Component)]
pub struct Hand{
    pub cards: (Card, Card)
}

//Need this Hands component in order to create a valid PlayerBundle
#[derive(Component)]
pub struct Hands(pub Vec<Hand>);

#[derive(Component)]
pub struct Card{
    pub suite: String,
    pub face: String,
    //Tuple used for implementing aces (1 or 11)
    pub value: (u8,u8), 
}
