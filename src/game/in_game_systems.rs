///in game systems module is used and responsible for creation and handling of UI components such as the buttons, cards, text elements, etc

use bevy::prelude::*;
use super::{components::{Card, ChipButtonValue, DealerHand, Deck, InGameCardAccess, PlayerBalance, PlayerButtonValues, PlayerHand, PlayerHands, TextComponents}, constants::{AppState, GameRoundState, CARD_HORIZONTAL_SPACING, CARD_VERTICAL_SPACING, DEALER_CARDS_INITIAL_HORIZONTAL_POSITION, DEALER_CARDS_INITIAL_VERTICAL_POSITION, PLAYER_CARDS_INITIAL_HORIZONTAL_POSITION, PLAYER_CARDS_INITIAL_VERTICAL_POSITION}, player_systems::{double_down_player_hand, hit_player_hand, stand_player_hand}, resources::{BalanceValue, BetValue, ParentNode}, traits::Dealable};

/// in_game_setup is the function used for setting up the base of our game once the start screen is bypassed.
/// We use it to spawn the player and dealer hands, as well as spawn the parent entity that all of our UI 
/// components are attached to. Finally, the buttons, chip buttons, text, and cards are spawned here with the help
/// of helper functions.
pub fn in_game_setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut parent_node: ResMut<ParentNode>,
    player_hands: Query<&PlayerHands>, 
    dealer_hands: Query<&DealerHand>,
    ) {
    
    // Safely fetch player hand
    let player_hand = match player_hands.get_single() {
        Ok(player_hands) => &player_hands.0[0],
        Err(_) => {
            println!("No player hands found, aborting setup");
            return;
        }
    };

    // Safely fetch dealer hand
    let dealer_hand = match dealer_hands.get_single() {
        Ok(dealer_hand) => dealer_hand,
        Err(_) => {
            println!("No dealer hands found, aborting setup");
            return;
        }
    };

    // Spawn UI parent entity, all children will be built off of this
    let parent_entity = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        
        //spawn the componenets of the UI at once via their helper functions
        spawn_buttons(parent, &assets);
        
        spawn_chip_buttons(parent, &assets);

        spawn_text_fields(parent, &assets);

        spawn_cards(parent, &assets, player_hand, dealer_hand);
    })
    .id();
    parent_node.0 = parent_entity;


}

//UI component functions below ----------------


/// spawn_image_button is the helper function we use to spawn the chip buttons (they use an image as the button background)
/// We made this because there are a handful of chip buttons, and this allows us to dynamically spawn them with specific 
/// parameters such as location and image to be attached, all without unnecessary repitition. 
fn spawn_image_button(
    parent: &mut ChildBuilder,
    assets: &Res<AssetServer>,
    position: Vec2,
    image_path: &'static str,
    value: ChipButtonValue,
) {
    parent.spawn(ButtonBundle {
        //style components
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
    //attach ChipButtonValue enum type here
    .insert(value)
    //attach image to button
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

/// spawn_text is our text helper function that allows us to spawn various text elements with different parameters and values.
/// This function and the other spawn functions all take in that original parent node that the UI is based upon, and build a child 
/// element that is attached to the main UI parent node.
fn spawn_text(
    parent: &mut ChildBuilder,
    assets: &Res<AssetServer>,
    position: Vec2,
    text: &str,
    font_size: f32,
    text_component: TextComponents,
) -> Entity {
    let entity = parent.spawn(TextBundle {
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
    })
    //insert TextComponent enum type for the text to be altered later
    .insert(text_component)
    .id();

    entity
}


/// spawn_buttons spawns our player system buttons such as stand, hit, double down, etc. Same as the other spawn functions, we use this 
/// to dynamically spawn these buttons with unique values to avoid unnecessary repetition.
fn spawn_buttons(parent: &mut ChildBuilder, assets: &Res<AssetServer>) {
    //setup individual buttons to be passed through the builder
    let button_positions = vec![
        (Vec2::new(5.0, 350.0), "Stand", 30.0, PlayerButtonValues::Stand),
        (Vec2::new(105.0, 350.0), "Hit", 30.0, PlayerButtonValues::Hit),
        (Vec2::new(205.0, 350.0), "Double Down", 15.0, PlayerButtonValues::DoubleDown),
        (Vec2::new(305.0, 350.0), "Deal", 30.0, PlayerButtonValues::Deal),
        (Vec2::new(690.0, 10.0), "Home", 15.0, PlayerButtonValues::Home),
    ];

    //build the buttons listed above ^
    for (position, label, font_size, button_value) in button_positions {
        let mut button_bundle = ButtonBundle {
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
        };

        //hiding and ensuring visibility for necessary buttons
        button_bundle.visibility = match button_value {
            PlayerButtonValues::Home => Visibility::Visible,
            _ => Visibility::Hidden,
        };

        parent.spawn(button_bundle)
            .insert(button_value)
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
}


///spawn_chip_buttons utilizes the spawn_image_button helper function and creates the chip buttons based on their unique values
fn spawn_chip_buttons(parent: &mut ChildBuilder, assets: &Res<AssetServer>) {
    spawn_image_button(parent, &assets, Vec2::new(5.0, 400.0), "chips/1.png", ChipButtonValue::One);
    spawn_image_button(parent, &assets, Vec2::new(105.0, 400.0), "chips/5.png", ChipButtonValue::Five);
    spawn_image_button(parent, &assets, Vec2::new(205.0, 400.0), "chips/10.png", ChipButtonValue::Ten);
    spawn_image_button(parent, &assets, Vec2::new(305.0, 400.0), "chips/50.png", ChipButtonValue::Fifty);
}

/// spawn_text_fields spawns all text elements for the UI, using the helper function with unique parameters per each text
fn spawn_text_fields(parent: &mut ChildBuilder, assets: &Res<AssetServer>) {
    spawn_text(parent, &assets, Vec2::new(125.0, 305.0), "Bet Amount:", 30.0, TextComponents::NotChanged);
    spawn_text(parent, &assets, Vec2::new(270.0, 305.0), "x", 30.0, TextComponents::Bet);
    spawn_text(parent, &assets, Vec2::new(15.0, 15.0), "User", 30.0, TextComponents::NotChanged);
    spawn_text(parent, &assets, Vec2::new(150.0, 15.0), "Balance:", 30.0, TextComponents::NotChanged);
    spawn_text(parent, &assets, Vec2::new(255.0, 15.0), "x", 30.0, TextComponents::Balance);
    spawn_text(parent, &assets, Vec2::new(415.0, 15.0), "Dealer", 30.0, TextComponents::NotChanged);
    spawn_text(parent, &assets, Vec2::new(40.0, 200.0), "Please place a bet then hit deal", 30.0, TextComponents::Instruction);
}

///spawn_result_text is uses to spawn win / loss statements once the round ends
pub fn spawn_result_text(
    parent: &mut ChildBuilder,
    assets: &Res<AssetServer>,
    result: &str,
) -> Entity {
    spawn_text(
        parent,
        assets,
        Vec2::new(10.0, 255.0), 
        result,
        50.0, 
        TextComponents::ResultText, 
    )
}

///spawn_player_card is a helper function used for spawning the player cards on the player side of the screen
pub fn spawn_player_card(
    parent: &mut ChildBuilder,
    assets: &Res<AssetServer>,
    card: &Card,
    card_index: usize,
    card_position: Vec2,
    is_visible: bool
) {
    parent.spawn(ImageBundle {
        style: Style {
            width: Val::Px(90.0),
            height: Val::Px(135.0),
            position_type: PositionType::Absolute,
            left: Val::Px(card_position.x),
            top: Val::Px(card_position.y),
            ..default()
        },
        image: UiImage {
            texture: assets.load(&card.front_asset_path),
            ..default()
        },
        visibility: if is_visible { Visibility::Visible } else { Visibility::Hidden },
        transform: Transform {
            translation: Vec3::new(card_position.x, card_position.y, card_index as f32 * 0.0001),
            ..default()
        },
        ..default()
    })
    .insert(InGameCardAccess::PlayerCard(card_index));
}

///spawn_player_cards utilizes spawn_player_card to go through the players hand and spawn cards in the hand
fn spawn_player_cards(parent: &mut ChildBuilder, assets: &Res<AssetServer>, player_hand: &PlayerHand) {
    for (i, card) in player_hand.cards.iter().enumerate() {
        spawn_player_card(
            parent,
            assets,
            card,
            i,
            Vec2 {
                x: PLAYER_CARDS_INITIAL_HORIZONTAL_POSITION + (i as f32)*CARD_HORIZONTAL_SPACING,
                y: PLAYER_CARDS_INITIAL_VERTICAL_POSITION  + (i as f32)*CARD_VERTICAL_SPACING}, 
            false
        );
    }
}

///spawn_dealer_card is a helper function used for spawning the dealer cards on the dealer side of the screen
pub fn spawn_dealer_card(
    parent: &mut ChildBuilder, 
    assets: &Res<AssetServer>, 
    card: &Card,
    card_index: usize, 
    card_position: Vec2,
    load_front_asset: bool,
    is_visible: bool){
    parent.spawn(ImageBundle {
        style: Style {
            width: Val::Px(90.0),
            height: Val::Px(135.0),
            position_type: PositionType::Absolute,
            left: Val::Px(card_position.x),
            top: Val::Px(card_position.y),
            ..default()
        },
        image: UiImage {
            texture: if load_front_asset{ assets.load(&card.front_asset_path)} else { assets.load(&card.back_asset_path)},
            ..default()
        },
        
        visibility: if is_visible {Visibility::Visible} else{Visibility::Hidden},
        ..default()
    })
    .insert(InGameCardAccess::DealerCard(card_index));
}

///spawn_dealer_cards utilizes spawn_dealer_card to go through the dealers hand and spawn cards in the hand
fn spawn_dealer_cards(parent: &mut ChildBuilder, assets: &Res<AssetServer>, dealer_hand: &DealerHand) {
    for (i, card) in dealer_hand.cards.iter().enumerate() {
            if i == 0 {
                spawn_dealer_card(
                    parent, 
                    assets,
                    card,
                    i,
                    Vec2 {
                        x: DEALER_CARDS_INITIAL_HORIZONTAL_POSITION + (i as f32)*CARD_HORIZONTAL_SPACING,
                        y: DEALER_CARDS_INITIAL_VERTICAL_POSITION + (i as f32)*CARD_VERTICAL_SPACING }, 
                    false,
                    false
                );
            }
            else {
                spawn_dealer_card(
                    parent,
                    assets,
                    card,
                    i,
                    Vec2 { 
                        x: DEALER_CARDS_INITIAL_HORIZONTAL_POSITION + (i as f32)*CARD_HORIZONTAL_SPACING,
                        y: DEALER_CARDS_INITIAL_VERTICAL_POSITION  + (i as f32)*CARD_VERTICAL_SPACING},
                    true,
                false);
            }
    }
}

///spawn_cards calls the spawn_player_cards and spawn_dealer_cards functions at once to spawn the appropriate cards in the appropriate places
fn spawn_cards(parent: &mut ChildBuilder, assets: &Res<AssetServer>, player_hand: &PlayerHand, dealer_hand: &DealerHand) {
    spawn_player_cards(parent, assets, player_hand);
    spawn_dealer_cards(parent, assets, dealer_hand);    
}

///spawn_keep_playing_button is used to spawn a button that allows the user to reset the match and keep playing once a round ends
pub fn spawn_keep_playing_button(
    parent: &mut ChildBuilder,
    assets: &Res<AssetServer>,
) {
    parent.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(90.0),
            height: Val::Px(50.0),
            position_type: PositionType::Absolute,
            left: Val::Px(405.0),
            top: Val::Px(350.0),
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
            "Keep Playing",
            TextStyle {
                font: assets.load("fonts/FiraSans-SemiBold.ttf"),
                font_size: 15.0,
                color: Color::WHITE,
            },
        ));
    })
    .insert(PlayerButtonValues::KeepPlaying); 
}

/// print_all_dealer_cards is a function available for testing to print the contents of the cards the dealer has in hand
pub fn print_all_dealer_cards(
    dealer_hand_query: Query<&DealerHand>,
) {
    for dealer_hand in dealer_hand_query.iter() {
        println!("Dealer has {} cards:", dealer_hand.cards.len());
        for (i, card) in dealer_hand.cards.iter().enumerate() {
            println!("Dealer Card {}: {} of {}", i, card.face, card.suite);   
        }
    }
}

//button click functions below -----------------------

/// chip_button_click_system is used for handling chip button clicks, whether its adjusting the player balance or bet being placed
pub fn chip_button_click_system(
    mut bet_value: ResMut<BetValue>,
    mut balance_value: ResMut<BalanceValue>,
    mut interaction_query: Query<(&Button, &mut Interaction, &ChipButtonValue)>,
    mut text_query: Query<(&TextComponents, &mut Text)>,
    mut deal_button_query: Query<(&PlayerButtonValues, &mut Visibility), With<PlayerButtonValues>>,

) {

    //begin interaction query , if certain chip interacted with, do something related to that chip value
    for (_, mut interaction, value) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match *value {
                    ChipButtonValue::One => {
                        if balance_value.value >= 1{
                            bet_value.value += 1;
                            balance_value.value -= 1;
                        } else {
                            println!("Insufficient value for bet of 1");
                        }
                    },
                    ChipButtonValue::Five => {
                        if balance_value.value >= 5{
                            bet_value.value += 5;
                            balance_value.value -= 5;
                        } else {
                            println!("Insufficient value for bet of 5");
                        }
                    },
                    ChipButtonValue::Ten => {
                        if balance_value.value >= 10{
                            bet_value.value += 10;
                            balance_value.value -= 10;
                        } else {
                            println!("Insufficient value for bet of 10");
                        }
                    },
                    ChipButtonValue::Fifty => {
                        if balance_value.value >= 50{
                            bet_value.value += 50;
                            balance_value.value -= 50;
                        } else {
                            println!("Insufficient value for bet of 50");
                        }
                    },
                }
                *interaction = Interaction::None;

                //query to find the deal button and set it to be visible once chip button is clicked
                for (button_value, mut visibility) in deal_button_query.iter_mut() {
                    if let PlayerButtonValues::Deal = *button_value {
                        *visibility = Visibility::Visible;
                    }
                }
            }
            _ => {}
        }

        //below are a few lines that are used for updating the bet amount text and player balance text as chips are interacted with
        let new_bet_text = bet_value.value.to_string();
        let new_balance_text  = balance_value.value.to_string();

        for (text_component, mut text) in text_query.iter_mut() {
            if let TextComponents::Bet = text_component {
                text.sections[0].value = new_bet_text.clone(); 
            }

            if let TextComponents::Balance = text_component {
                text.sections[0].value = new_balance_text.clone();
            }
        }


    }

}


/// player_button_system is used to handle clicks on the player system buttons such as hit, stand, double down, etc.
pub fn player_button_system(
    mut next_state: ResMut<NextState<GameRoundState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    //param set created to allow us to avoid conflict while querying on visibility for multiple components 
    mut param_set: ParamSet<(
        Query<(&Button, &mut Interaction, &PlayerButtonValues, &mut Visibility), With<Button>>,
        Query<(&InGameCardAccess, &mut Visibility)>,       
        Query<(&TextComponents, &mut Visibility)>,      
        Query<(&ChipButtonValue, &mut Visibility)>,
        Query<(&PlayerButtonValues, &mut Visibility)>, 

    )>,    
) {
    let mut deal_button_pressed = false;
    let mut keep_playing_button_pressed = false;
    
    //interaction query for the playerbutton values, do something in particular if certain button pressed...
    for (_, mut interaction, value, _) in param_set.p0().iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match *value {
                    PlayerButtonValues::Home => {
                        println!("Home");
                        next_app_state.set(AppState::Start);
                        *interaction = Interaction::None;
                        
                    },
                    PlayerButtonValues::Deal => {
                        deal_button_pressed = true;
                        *interaction = Interaction::None;
                    },   
                    PlayerButtonValues::KeepPlaying => {
                        println!("Keep Playing Button Pressed");
                        keep_playing_button_pressed = true;
                        *interaction = Interaction::None;
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }

    //flags used to allow us to break logic for button press outside of the query for loop , as ownership and access is compromised 
    //if used in the query for loop.
    if keep_playing_button_pressed {

        //if keep playing button is pressed, we query for necessary components to be reset and visibility restored/revoked
        for (value, mut visibility) in param_set.p4().iter_mut() {
            match *value {
                PlayerButtonValues::KeepPlaying => {
                    *visibility = Visibility::Hidden;
                }
                _ => {}
            }
        }
        
        for (value, mut visibility) in param_set.p2().iter_mut() {
            match *value {
                TextComponents::ResultText => {
                    *visibility = Visibility::Hidden;
                }
                _ => {}
            }
        }

        next_state.set(GameRoundState::Betting);
    }

    //if deal button pressed, toggle necessary components visibility
    if deal_button_pressed {
        for (_, _, value,  mut visibility) in param_set.p0().iter_mut() {
            match *value {
                PlayerButtonValues::Deal => {
                    *visibility = Visibility::Hidden; 
                }
                PlayerButtonValues::Stand | PlayerButtonValues::Hit | PlayerButtonValues::DoubleDown => {
                    *visibility = Visibility::Visible; 
                }
                _ => {}
            }
        }

        for(chip_value, mut visibility) in param_set.p3().iter_mut() {
            match *chip_value {
                ChipButtonValue::One | ChipButtonValue::Five | ChipButtonValue::Ten | ChipButtonValue::Fifty => {
                    *visibility = Visibility::Hidden;
                }
            }
        }

        for (_, mut card_visibility) in param_set.p1().iter_mut() {
            *card_visibility = Visibility::Visible;
        }

        for (value, mut text_visibility) in param_set.p2().iter_mut() {
            match *value {
                TextComponents::Instruction => {
                    *text_visibility = Visibility::Hidden;
                }
                _ => {}
            } 
        }
        next_state.set(GameRoundState::PlayerHand);
    }
}

// game state related functions below ------------------------------------

///track_game_state used for testing purposes / to track game state as certain actions and turns are finished / started
pub fn track_game_state(game_state: Res<State<GameRoundState>>){
    let game_state_string = match game_state.get() {
        GameRoundState::RoundStart => "Round Start",
        GameRoundState::Betting => "Betting",
        GameRoundState::PlayerHand => "Player Hand",
        GameRoundState::DealerHand => "Dealer Hand",
        GameRoundState::RoundEnd => "Round End",
    };
    println!("Game State: {game_state_string}");
}

///track_app_state used for testing purposes / to track app state being in the start menu or ingame UI
pub fn track_app_state(current_app_state: Res<State<AppState>>) {
    let app_state_string = match current_app_state.get(){
        AppState::Start => "Start",
        AppState::InGame => "In Game",
    };
    println!("Current app state: {app_state_string}");
}

///reset_game is to be used for resetting values, hands, and UI components for when the user finishes a round and wants to play another.
/// currently it is not fully functional, everything except for the previous player and dealer hands are being reset.
/// attempted to reset player and dealer hands and did not find success, would love to give it a shot again when time allows.
pub fn reset_game(mut balance_value: ResMut<BalanceValue>, 
    mut bet_value: ResMut<BetValue>,
    mut next_state: ResMut<NextState<GameRoundState>>,
    // mut dealer_hand_query: Query<&mut DealerHand>,
    // mut player_hands_query: Query<&mut PlayerHands>,
    // mut cards_query: Query<Entity, With<InGameCardAccess>> ,
    // game_round_state: Res<State<GameRoundState>>,
    // mut commands: Commands,
    // mut next_app_state: ResMut<NextState<AppState>>,

) {
    balance_value.value = 1000;  
    bet_value.value = 0;         
    println!("Player balance reset to 1000 and bet reset to 0");

    next_state.set(GameRoundState::RoundStart);

    // for mut dealer_hand in dealer_hand_query.iter_mut() {
    //     dealer_hand.cards.clear();  
    // }

    // for mut player_hands in player_hands_query.iter_mut() {
    //     player_hands.0.clear(); 
    // }

    // for card in cards_query.iter_mut() {
    //     commands.entity(card).despawn()
    // }
}

/// despawn_cards_and_reset was another attempt to clear the player and dealer hands in between rounds.
/// odd behavior when clearing the player and dealer hands in the commented out code below.
pub fn despawn_cards_and_reset(
    mut commands: Commands,
    mut player_query: Query<(&mut PlayerHands, &mut PlayerBalance)>,
    mut deck: ResMut<Deck>,
    assets: Res<AssetServer>,
    cards_query: Query<Entity, With<InGameCardAccess>>,
    // dealer_query: Query<Entity, With<InGameCardAccess>>,
    mut parent_node: ResMut<ParentNode>, 
    // assets: Res<AssetServer>,
    mut player_hands: Query<&mut PlayerHands>,
    mut dealer_hands: Query<&mut DealerHand>,
    
) {

    // // maybe an array instead of vector for player hand cards?


    // for entity in cards_query.iter() {
    //     // commands.entity(parent_node.0).remove_children(&[entity]);
    //     commands.entity(entity).despawn_recursive();
    // }
    // if let Some(mut player_hands) = player_hands.iter_mut().next() {
    //     player_hands.0.clear(); 
    // }

    // if let Some(mut dealer_hands) = dealer_hands.iter_mut().next() {
    //     dealer_hands.cards.clear(); 
    // }

    // let (mut player_hands, _) = player_query.single_mut(); 
    // let player_hand = &mut player_hands.0[0];
    // let insert_index = player_hand.cards.len();
    // let card_to_insert = deck.deal();
    // player_hand.cards.push(card_to_insert.clone());
    // let position = Vec2 {
    //     x: PLAYER_CARDS_INITIAL_HORIZONTAL_POSITION + (insert_index as f32)*CARD_HORIZONTAL_SPACING,
    //     y: PLAYER_CARDS_INITIAL_VERTICAL_POSITION  + (insert_index as f32)*CARD_VERTICAL_SPACING};
    
    
    // commands.entity(parent_node.0).with_children(|parent|{
    //     spawn_player_card(
    //         parent,
    //         &assets, 
    //         &card_to_insert, 
    //         insert_index, 
    //         position,
    //         true,
    //     );
    // });


}


// function attempts to be deleted / cleaned / reused below --------------------


// fn spawn_cards_for_new_round(
//     parent: &mut ChildBuilder,
//     assets: &Res<AssetServer>,
//     player_hand: &PlayerHand,
//     dealer_hand: &DealerHand
// ) {
//     spawn_player_cards(parent, assets, player_hand);
//     spawn_dealer_cards(parent, assets, dealer_hand);
// }


// pub fn despawn_cards(
//     game_round_state: Res<State<GameRoundState>>,
//     mut commands: Commands,
//     player_query: Query<Entity, With<PlayerHand>>,
//     dealer_query: Query<Entity, With<DealerHand>>,
// ) {

//     if *game_round_state == GameRoundState::RoundEnd {
        
//         // Despawn all player hand entities (cards)
//         for entity in player_query.iter() {
//             commands.entity(entity).despawn();
//         }

//         // Despawn all dealer hand entities (cards)
//         for entity in dealer_query.iter() {
//             commands.entity(entity).despawn();
//         }
//     }
// }