use bevy::prelude::*;
use super::{components::{ChipButtonValue, DealerHand, InGameCardAccess, PlayerBalance, PlayerButtonValues, PlayerHand, PlayerHands, TextComponents}, player_systems::{double_down_player_hand, hit_player_hand, stand_player_hand}, resources::{BalanceValue, BetValue}};

pub fn in_game_setup(mut commands: Commands, assets: Res<AssetServer>, player_hands: Query<&PlayerHands>, dealer_hands: Query<&DealerHand>) {
    
    let player_hand = &player_hands.single().0[0]; 
    let dealer_hand = dealer_hands.single();

    commands.spawn(NodeBundle {
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
    });
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
            PlayerButtonValues::Deal => Visibility::Visible,
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

//spawning card images
fn spawn_player_cards(parent: &mut ChildBuilder, assets: &Res<AssetServer>, player_hand: &PlayerHand) {

    // if hit... add 3rd card and update positions somehow...

    let card_positions = vec![
        Vec2::new(110.0, 100.0),
        Vec2::new(210.0, 100.0), 
    ];

    for (i, card) in player_hand.cards.iter().enumerate() {

        if let Some(position ) = card_positions.get(i) {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(90.0),
                    height: Val::Px(135.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(position.x),
                    top: Val::Px(position.y),
                    ..default()
                },
                image: UiImage {
                    texture: assets.load(&card.front_asset_path),
                    ..default()
                },
                
                visibility: Visibility::Hidden,
                ..default()
            })
            .insert(InGameCardAccess::PlayerCard1);

        }
    }
}

fn spawn_dealer_cards(parent: &mut ChildBuilder, assets: &Res<AssetServer>, dealer_hand: &DealerHand) {

    let card_positions = vec![
        Vec2::new(500.0, 100.0),
        Vec2::new(600.0, 100.0), 
    ];

    for (i, card) in dealer_hand.cards.iter().enumerate() {

        if let Some(position) = card_positions.get(i) {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(90.0),
                    height: Val::Px(135.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(position.x),
                    top: Val::Px(position.y),
                    ..default()
                },
                image: UiImage {
                    texture: assets.load(&card.front_asset_path),
                    ..default()
                },
                
                visibility: Visibility::Hidden,
                ..default()
            })
            .insert(InGameCardAccess::DealerCard1);

        }    
    }
}

fn spawn_cards(parent: &mut ChildBuilder, assets: &Res<AssetServer>, player_hand: &PlayerHand, dealer_hand: &DealerHand) {
    spawn_player_cards(parent, assets, player_hand);
    spawn_dealer_cards(parent, assets, dealer_hand);
}

//button click functions below -----------------------

//dealing with chip button clicks
pub fn chip_button_click_system(
    mut bet_value: ResMut<BetValue>,
    mut balance_value: ResMut<BalanceValue>,
    mut interaction_query: Query<(&Button, &mut Interaction, &ChipButtonValue)>,
    mut text_query: Query<(&TextComponents, &mut Text)>,
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

pub fn player_button_system(
    mut player_query: Query<(&mut PlayerHands, &mut PlayerBalance)>,
    mut dealer_query: Query<&mut DealerHand>,
    mut bet_value: ResMut<BetValue>, 
    mut player_balance_result: ResMut<BalanceValue>, 
    mut param_set: ParamSet<(
        Query<(&Button, &mut Interaction, &PlayerButtonValues, &mut Visibility), With<Button>>,
        Query<(&InGameCardAccess, &mut Visibility)>,       
        Query<(&TextComponents, &mut Visibility)>,      
        Query<(&ChipButtonValue, &mut Visibility)>  
    )>,  
) {
    let mut hit_button_pressed = false;
    let mut stand_button_pressed = false;
    let mut double_button_pressed = false;
    let mut deal_button_pressed = false;
    
    for (_, mut interaction, value, mut visibility) in param_set.p0().iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match *value {
                    PlayerButtonValues::Stand => stand_button_pressed = true,
                    PlayerButtonValues::Hit => hit_button_pressed = true,
                    PlayerButtonValues::DoubleDown => double_button_pressed = true,
                    PlayerButtonValues::Home => println!("Home"),
                    PlayerButtonValues::Deal => deal_button_pressed = true,   
                }
                *interaction = Interaction::None;
            }
            _ => {}
        }
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
    }
    else if stand_button_pressed {
        stand_player_hand(player_query);
    }
    else if hit_button_pressed {
        hit_player_hand(player_query);
    }
    else if double_button_pressed {
        double_down_player_hand(player_query);
    }
    
}
