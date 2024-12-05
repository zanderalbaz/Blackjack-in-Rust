use bevy::prelude::*;
use super::{components::{BetAmountText, ChipButtonValue, PlayerBalance, PlayerButtonValues, PlayerHands}, player_systems::{double_down_player_hand, hit_player_hand, stand_player_hand}};

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

fn spawn_text(
    parent: &mut ChildBuilder,
    assets: &Res<AssetServer>,
    position: Vec2,
    text: &str,
    font_size: f32,
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
    }).id();

    entity
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
        
        spawn_buttons(parent, &assets);
        
        spawn_image_buttons(parent, &assets);

        spawn_text_fields(parent, &assets);

        spawn_cards(parent, &assets);
    });
}

fn spawn_buttons(parent: &mut ChildBuilder, assets: &Res<AssetServer>) {
    let button_positions = vec![
        (Vec2::new(5.0, 350.0), "Stand", 30.0, PlayerButtonValues::Stand),
        (Vec2::new(105.0, 350.0), "Hit", 30.0, PlayerButtonValues::Hit),
        (Vec2::new(205.0, 350.0), "Double Down", 15.0, PlayerButtonValues::DoubleDown),
        (Vec2::new(690.0, 10.0), "Home", 15.0, PlayerButtonValues::Home),
    ];

    for (position, label, font_size, button_value) in button_positions {
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


fn spawn_image_buttons(parent: &mut ChildBuilder, assets: &Res<AssetServer>) {
    spawn_image_button(parent, &assets, Vec2::new(5.0, 400.0), "chips/1.png", ChipButtonValue::One);
    spawn_image_button(parent, &assets, Vec2::new(105.0, 400.0), "chips/5.png", ChipButtonValue::Five);
    spawn_image_button(parent, &assets, Vec2::new(205.0, 400.0), "chips/10.png", ChipButtonValue::Ten);
    spawn_image_button(parent, &assets, Vec2::new(305.0, 400.0), "chips/50.png", ChipButtonValue::Fifty);
}

fn spawn_text_fields(parent: &mut ChildBuilder, assets: &Res<AssetServer>) {
    spawn_text(parent, &assets, Vec2::new(125.0, 305.0), "Bet Amount:", 30.0);
    spawn_text(parent, &assets, Vec2::new(270.0, 305.0), "x", 30.0);

    /*
        !! can't get this to work due to borrowing commands one too many times...  !!

        -------------

        let bet_entity = spawn_text(parent, &assets, Vec2::new(270.0, 305.0), "x", 30.0);

    
        commands.entity(bet_entity).insert(BetAmountText {
            bet_text: "10".to_string(),
            balance_text: "100".to_string(),
        });

        ------------
     */
    

    spawn_text(parent, &assets, Vec2::new(15.0, 15.0), "User", 30.0);
    spawn_text(parent, &assets, Vec2::new(150.0, 15.0), "Balance:", 30.0);
    spawn_text(parent, &assets, Vec2::new(255.0, 15.0), "x", 30.0);
    spawn_text(parent, &assets, Vec2::new(415.0, 15.0), "Dealer", 30.0);
}

fn spawn_cards(parent: &mut ChildBuilder, assets: &Res<AssetServer>) {
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
}

fn update_bet_text(bet_amount_entity: Entity, new_bet_text: String, mut query: Query<(&mut BetAmountText, &mut Text)>) {
    if let Ok((mut text, _)) = query.get_mut(bet_amount_entity) {
        text.bet_text = new_bet_text;
    }
}

#[derive(Resource,Default)]
pub struct BetValue {
    pub value: i32,
}

pub fn chip_button_click_system(

    /*
        This works, but i can't get it to update the correct text value yet due to a borrow issue above...
     */
    mut bet_value: ResMut<BetValue>,
    mut interaction_query: Query<(&Button, &mut Interaction, &ChipButtonValue)>,
) {
    for (_, mut interaction, value) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match *value {
                    ChipButtonValue::One => bet_value.value += 1,
                    ChipButtonValue::Five => bet_value.value += 5,
                    ChipButtonValue::Ten => bet_value.value += 10,
                    ChipButtonValue::Fifty => bet_value.value += 50,
                }
                *interaction = Interaction::None;
            }
            _ => {}
        }
    }
    //println!("Current Bet Value: {}", bet_value.value);
}

#[derive(Resource,Default)]
pub struct BalanceValue {
    pub value: i32,
}

pub fn player_button_system(
    mut interaction_query: Query<(&Button, &mut Interaction, &PlayerButtonValues), With<Button>>,
    mut player_query: Query<(&mut PlayerHands, &mut PlayerBalance)>,
    mut bet_value: ResMut<BetValue>, 
    mut player_balance_result: ResMut<BalanceValue>, 
) {
    let mut hit_button_pressed = false;
    let mut stand_button_pressed = false;
    let mut double_button_pressed = false;
    let mut home_button_pressed = false;

    for (_, mut interaction, value) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match *value {
                    PlayerButtonValues::Stand => stand_button_pressed = true,
                    PlayerButtonValues::Hit => hit_button_pressed = true,
                    PlayerButtonValues::DoubleDown => double_button_pressed = true,
                    PlayerButtonValues::Home => println!("Home"),
                    
                }
                *interaction = Interaction::None;
            }
            _ => {}
        }
    }

    if(stand_button_pressed) {
        stand_player_hand(player_query);
    }
    else if (hit_button_pressed) {
        hit_player_hand(player_query);
    }
    else if (double_button_pressed) {
        double_down_player_hand(player_query);
    }
    

   
}
