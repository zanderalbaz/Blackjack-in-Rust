use bevy::prelude::*;
use super::{components::{Card, ChipButtonValue, DealerHand, InGameCardAccess, PlayerBalance, PlayerButtonValues, PlayerHand, PlayerHands, TextComponents}, constants::{AppState, GameRoundState, CARD_HORIZONTAL_SPACING, CARD_VERTICAL_SPACING, DEALER_CARDS_INITIAL_HORIZONTAL_POSITION, DEALER_CARDS_INITIAL_VERTICAL_POSITION, PLAYER_CARDS_INITIAL_HORIZONTAL_POSITION, PLAYER_CARDS_INITIAL_VERTICAL_POSITION}, player_systems::{double_down_player_hand, hit_player_hand, stand_player_hand}, resources::{BalanceValue, BetValue, ParentNode}};

pub fn in_game_setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut parent_node: ResMut<ParentNode>,
    player_hands: Query<&PlayerHands>, 
    dealer_hands: Query<&DealerHand>,
    ) {
    
    let player_hand = match player_hands.get_single() {
        Ok(player_hands) => &player_hands.0[0], // Assuming player hands exist
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

    let parent_entity = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        
        spawn_buttons(parent, &assets);
        
        spawn_chip_buttons(parent, &assets);

        spawn_text_fields(parent, &assets);

        spawn_cards(parent, &assets, player_hand, dealer_hand);
    })
    .id();
    parent_node.0 = parent_entity;


}

//UI component functions below ----------------

//spawning chip buttons dynamically
fn spawn_image_button(
    parent: &mut ChildBuilder,
    assets: &Res<AssetServer>,
    position: Vec2,
    image_path: &'static str,
    value: ChipButtonValue,
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
    .insert(value)
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

//spawning text elements dynamically
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
    .insert(text_component)
    .id();

    entity
}

//spawning player game buttons dynamically
fn spawn_buttons(parent: &mut ChildBuilder, assets: &Res<AssetServer>) {
    let button_positions = vec![
        (Vec2::new(5.0, 350.0), "Stand", 30.0, PlayerButtonValues::Stand),
        (Vec2::new(105.0, 350.0), "Hit", 30.0, PlayerButtonValues::Hit),
        (Vec2::new(205.0, 350.0), "Double Down", 15.0, PlayerButtonValues::DoubleDown),
        (Vec2::new(305.0, 350.0), "Deal", 30.0, PlayerButtonValues::Deal),
        (Vec2::new(690.0, 10.0), "Home", 15.0, PlayerButtonValues::Home),
    ];

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

//spawning the chip buttons with specific values 
fn spawn_chip_buttons(parent: &mut ChildBuilder, assets: &Res<AssetServer>) {
    spawn_image_button(parent, &assets, Vec2::new(5.0, 400.0), "chips/1.png", ChipButtonValue::One);
    spawn_image_button(parent, &assets, Vec2::new(105.0, 400.0), "chips/5.png", ChipButtonValue::Five);
    spawn_image_button(parent, &assets, Vec2::new(205.0, 400.0), "chips/10.png", ChipButtonValue::Ten);
    spawn_image_button(parent, &assets, Vec2::new(305.0, 400.0), "chips/50.png", ChipButtonValue::Fifty);
}

//spawning the text elements with specific values
fn spawn_text_fields(parent: &mut ChildBuilder, assets: &Res<AssetServer>) {
    spawn_text(parent, &assets, Vec2::new(125.0, 305.0), "Bet Amount:", 30.0, TextComponents::NotChanged);
    spawn_text(parent, &assets, Vec2::new(270.0, 305.0), "x", 30.0, TextComponents::Bet);
    spawn_text(parent, &assets, Vec2::new(15.0, 15.0), "User", 30.0, TextComponents::NotChanged);
    spawn_text(parent, &assets, Vec2::new(150.0, 15.0), "Balance:", 30.0, TextComponents::NotChanged);
    spawn_text(parent, &assets, Vec2::new(255.0, 15.0), "x", 30.0, TextComponents::Balance);
    spawn_text(parent, &assets, Vec2::new(415.0, 15.0), "Dealer", 30.0, TextComponents::NotChanged);
    spawn_text(parent, &assets, Vec2::new(40.0, 200.0), "Please place a bet then hit deal", 30.0, TextComponents::Instruction);
}

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

pub fn spawn_dealer_card(
    parent: &mut ChildBuilder, 
    assets: &Res<AssetServer>, 
    card: &Card,
    card_index: usize, 
    card_position: Vec2,
    load_front_asset: bool,
    is_visible: bool){
    // println!("running spawn_dealer_card for {} of {} with asset: {} at position: {}", card.face, card.suite, card.front_asset_path, card_index);
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

fn spawn_cards(parent: &mut ChildBuilder, assets: &Res<AssetServer>, player_hand: &PlayerHand, dealer_hand: &DealerHand) {
    spawn_player_cards(parent, assets, player_hand);
    spawn_dealer_cards(parent, assets, dealer_hand);    
}

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

//dealing with chip button clicks
pub fn chip_button_click_system(
    mut bet_value: ResMut<BetValue>,
    mut balance_value: ResMut<BalanceValue>,
    mut interaction_query: Query<(&Button, &mut Interaction, &ChipButtonValue)>,
    mut text_query: Query<(&TextComponents, &mut Text)>,
    mut deal_button_query: Query<(&PlayerButtonValues, &mut Visibility), With<PlayerButtonValues>>,

) {
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

                for (button_value, mut visibility) in deal_button_query.iter_mut() {
                    if let PlayerButtonValues::Deal = *button_value {
                        *visibility = Visibility::Visible;
                    }
                }
            }
            _ => {}
        }

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

//dealing with player game button clicks

/// this is the player button system function

pub fn player_button_system(
    mut next_state: ResMut<NextState<GameRoundState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
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
    if keep_playing_button_pressed {
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

pub fn track_app_state(current_app_state: Res<State<AppState>>) {
    let app_state_string = match current_app_state.get(){
        AppState::Start => "Start",
        AppState::InGame => "In Game",
    };
    println!("Current app state: {app_state_string}");
}

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

pub fn despawn_cards_and_reset(
    mut commands: Commands,
    cards_query: Query<Entity, With<InGameCardAccess>>,
    // dealer_query: Query<Entity, With<InGameCardAccess>>,
    // mut parent_node: ResMut<ParentNode>, 
    // assets: Res<AssetServer>,
    mut player_hands: Query<&mut PlayerHands>,
    mut dealer_hands: Query<&mut DealerHand>,
    
) {

    // maybe an array instead of vector for player hand cards?
    

    // if let Some(mut player_hands) = player_hands.iter_mut().next() {
    //     player_hands.0.clear(); 
    // }

    // if let Some(mut dealer_hands) = dealer_hands.iter_mut().next() {
    //     dealer_hands.cards.clear(); 
    // }

    for entity in cards_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
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