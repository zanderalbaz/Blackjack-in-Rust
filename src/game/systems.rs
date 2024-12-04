use bevy::prelude::*;
use super::{bundles::DealerBundle, components::{Card, DealerHand, Decks, PlayerBalance, PlayerHand, PlayerHands, PlayerName, PressEnterToPlay}, constants::AppState};
use super::bundles::PlayerBundle;

//TODO: Refactor this file into multiple files for orginization


pub fn start_game(mut query: Query<&mut Transform, With<PressEnterToPlay>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,

){
    if keyboard_input.just_pressed(KeyCode::Enter) {

        next_state.set(AppState::InGame);

    }
}

fn spawn_button_with_text(
    parent: &mut ChildBuilder,
    assets: &Res<AssetServer>,
    position: Vec2,
    label: &str,
    font_size: f32,
) {
    parent.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(90.0),
            height: Val::Px(50.0),
            position_type: PositionType::Absolute,
            left: Val::Px(position.x),
            top: Val::Px(position.y),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect { left: Val::Px(10.0), bottom: Val::Px(10.0), ..default() },
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font: assets.load("fonts/FiraSans-SemiBold.ttf"),
                font_size,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        ));
    });
}

fn spawn_image_button(
    parent: &mut ChildBuilder,
    assets: &Res<AssetServer>,
    position: Vec2,
    image_path: &'static str,
) {
    parent.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(90.0),
            height: Val::Px(90.0),
            position_type: PositionType::Absolute,
            left: Val::Px(position.x),
            top: Val::Px(position.y),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect { left: Val::Px(10.0), bottom: Val::Px(10.0), ..default() },
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(ImageBundle {
            style: Style {
                width: Val::Px(90.0),
                height: Val::Px(90.0),
                ..default()
            },
            image: UiImage {
                texture: assets.load(image_path),
                ..default()
            },
            ..default()
        });
    });
}

fn spawn_text(
    parent: &mut ChildBuilder,
    assets: &Res<AssetServer>,
    position: Vec2,
    text: &str,
    font_size: f32,
) {
    parent.spawn(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: text.to_string(),
                    style: TextStyle {
                        font: assets.load("fonts/FiraSans-SemiBold.ttf"),
                        font_size,
                        color: Color::WHITE,
                    },
                },
            ],
            ..default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Px(position.x),
            top: Val::Px(position.y),
            ..default()
        },
        ..default()
    });
}


pub fn inGame_setup(mut commands: Commands, assets: Res<AssetServer>) {

    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        let button_positions = vec![
            (Vec2::new(5.0, 350.0), "Stand", 30.0),
            (Vec2::new(105.0, 350.0), "Hit", 30.0),
            (Vec2::new(205.0, 350.0), "Split", 30.0),
            (Vec2::new(305.0, 350.0), "Double Down", 15.0),
            (Vec2::new(690.0, 10.0), "Home", 15.0),
        ];

        for (position, label, font_size) in button_positions {
            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(90.0),
                    height: Val::Px(50.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(position.x),
                    top: Val::Px(position.y),
                    border: UiRect::all(Val::Px(5.0)), 
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect { left: Val::Px(10.0), bottom: Val::Px(10.0), ..default() },
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    label,
                    TextStyle {
                        font: assets.load("fonts/FiraSans-SemiBold.ttf"),
                        font_size,
                        color: Color::srgb(0.9, 0.9, 0.9),
                    },
                ));
            });
        }

        let image_button_positions = vec![
            (Vec2::new(5.0, 400.0), "chips/1.png"),
            (Vec2::new(105.0, 400.0), "chips/5.png"),
            (Vec2::new(205.0, 400.0), "chips/10.png"),
            (Vec2::new(305.0, 400.0), "chips/50.png"),
        ];

        for (position, image_path) in image_button_positions {
            spawn_image_button(parent, &assets, position, image_path);
        }

        spawn_text(parent, &assets, Vec2::new(125.0, 305.0), "Bet Amount: x", 30.0);
        spawn_text(parent, &assets, Vec2::new(15.0, 15.0), "User", 30.0);
        spawn_text(parent, &assets, Vec2::new(150.0, 15.0), "Balance: x", 30.0);
        spawn_text(parent, &assets, Vec2::new(415.0, 15.0), "Dealer", 30.0);

        let card_positions = vec![
            (Vec2::new(50.0, 100.0), "deck/2_of_clubs.png"),
            (Vec2::new(205.0, 100.0), "deck/king_of_hearts.png"),
            (Vec2::new(450.0, 100.0), "deck/card_back.png"),
            (Vec2::new(605.0, 100.0), "deck/card_back.png"),
        ];

        for (position, card_image) in card_positions {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(195.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(position.x),
                    top: Val::Px(position.y),
                    ..default()
                },
                image: UiImage {
                    texture: assets.load(card_image),
                    ..default()
                },
                ..default()
            });
        }
    });
}


//TODO: Split the player functions below into a new file (This file is gonna be fucking huge if not lol)

pub fn spawn_test_player(mut commands: Commands){
    commands.spawn(PlayerBundle{
        player_name: PlayerName(String::from("test").into()),
        player_balance: PlayerBalance(100.),
        player_hands: PlayerHands(vec![PlayerHand{
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

pub fn test_player_balance_change(mut query: Query<&mut PlayerBalance>){
    for mut balance in &mut query{
        println!("Player has balance of {}", balance.0);
        balance.0 += 1.;
        println!("Player has updated balance of {}", balance.0);
    }
}

//TODO: Implement system to update the player balance on GUI


//TODO: Implement system to update the player username on GUI


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


pub fn test_player_hand(mut query: Query<&mut PlayerHands>){
    for player_hand in &mut query{
        let card1 = &player_hand.0[0].cards[0];
        let card2 = &player_hand.0[0].cards[1];
        println!("Cards: {} of {}, {} of {}", card1.face, card1.suite, card2.face, card2.suite);
    }
}



//TODO: Split the dealer functions below into a new file

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

