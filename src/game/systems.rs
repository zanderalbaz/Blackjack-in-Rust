use bevy::prelude::*;
use super::{components::{PlayerName, PressEnterToPlay, Balance, Hands, Card, Hand}, constants::AppState};
use super::bundles::PlayerBundle;

pub fn start_game(mut query: Query<&mut Transform, With<PressEnterToPlay>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,

){
    if keyboard_input.just_pressed(KeyCode::Enter) {

        next_state.set(AppState::InGame);

    }
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
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(90.0),
                height: Val::Px(50.0),
                position_type: PositionType::Absolute, 
                left: Val::Px(5.0),   
                top: Val::Px(350.0),  
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect {left: Val::Px(10.0), bottom: Val::Px(10.0), ..default()},
                ..default()

            },
            border_color: BorderColor(Color::BLACK),
            background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Stand",
             TextStyle {
                font: assets.load("fonts/FiraSans-SemiBold.ttf"),
                font_size: 30.0,
                color: Color::srgb(0.9, 0.9, 0.9),
             }
            ));
        });
    })
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(90.0),
                height: Val::Px(50.0),
                position_type: PositionType::Absolute, 
                left: Val::Px(105.0),   
                top: Val::Px(350.0),  
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect {left: Val::Px(10.0), bottom: Val::Px(10.0), ..default()},
                ..default()

            },
            border_color: BorderColor(Color::BLACK),
            background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Hit",
             TextStyle {
                font: assets.load("fonts/FiraSans-SemiBold.ttf"),
                font_size: 30.0,
                color: Color::srgb(0.9, 0.9, 0.9),
             }
            ));
        });
    })
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(90.0),
                height: Val::Px(50.0),
                position_type: PositionType::Absolute, 
                left: Val::Px(305.0),   
                top: Val::Px(350.0),  
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect {left: Val::Px(10.0), bottom: Val::Px(10.0), ..default()},
                ..default()

            },
            border_color: BorderColor(Color::BLACK),
            background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Split",
             TextStyle {
                font: assets.load("fonts/FiraSans-SemiBold.ttf"),
                font_size: 30.0,
                color: Color::srgb(0.9, 0.9, 0.9),
             }
            ));
        });
    })
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(90.0),
                height: Val::Px(50.0),
                position_type: PositionType::Absolute, 
                left: Val::Px(205.0),   
                top: Val::Px(350.0),  
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect {left: Val::Px(10.0), bottom: Val::Px(10.0), ..default()},
                ..default()

            },
            border_color: BorderColor(Color::BLACK),
            background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Double Down",
             TextStyle {
                font: assets.load("fonts/FiraSans-SemiBold.ttf"),
                font_size: 15.0,
                color: Color::srgb(0.9, 0.9, 0.9),
             }
            ));
        });
    })
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(90.0),
                height: Val::Px(90.0),
                position_type: PositionType::Absolute, 
                left: Val::Px(5.0),   
                top: Val::Px(400.0),  
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect {left: Val::Px(10.0), bottom: Val::Px(10.0), ..default()},
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
                image: UiImage{ 
                    texture: assets.load("chips/1.png"),
                    ..default()
                },
                ..default() 
            });
        });
    })
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(90.0),
                height: Val::Px(90.0),
                position_type: PositionType::Absolute, 
                left: Val::Px(105.0),   
                top: Val::Px(400.0),  
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect {left: Val::Px(10.0), bottom: Val::Px(10.0), ..default()},
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
                image: UiImage{ 
                    texture: assets.load("chips/5.png"),
                    ..default()
                },
                ..default() 
            });
        });
    })
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(90.0),
                height: Val::Px(90.0),
                position_type: PositionType::Absolute, 
                left: Val::Px(205.0),   
                top: Val::Px(400.0),  
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect {left: Val::Px(10.0), bottom: Val::Px(10.0), ..default()},
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
                image: UiImage{ 
                    texture: assets.load("chips/10.png"),
                    ..default()
                },
                ..default() 
            });
        });
    })
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(90.0),
                height: Val::Px(90.0),
                position_type: PositionType::Absolute, 
                left: Val::Px(305.0),   
                top: Val::Px(400.0),  
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect {left: Val::Px(10.0), bottom: Val::Px(10.0), ..default()},
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
                image: UiImage{ 
                    texture: assets.load("chips/50.png"),
                    ..default()
                },
                ..default() 
            });
        });
    })
    .with_children(|parent| {
        
        parent.spawn(ImageBundle {
            style: Style {
                width: Val::Px(150.0), 
                height: Val::Px(195.0),
                position_type: PositionType::Absolute,
                left: Val::Px(50.0),  
                top: Val::Px(100.0),   
                ..default()
            },
            image: UiImage {
                texture: assets.load("deck/2_of_clubs.png"),  
                ..default()
            },
            ..default()
        });
    })
    .with_children(|parent| {
        
        parent.spawn(ImageBundle {
            style: Style {
                width: Val::Px(150.0), 
                height: Val::Px(195.0),
                position_type: PositionType::Absolute,
                left: Val::Px(205.0),  
                top: Val::Px(100.0),   
                ..default()
            },
            image: UiImage {
                texture: assets.load("deck/king_of_hearts.png"),  
                ..default()
            },
            ..default()
        });
    })
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Bet Amount: x".to_string(),
                        style: TextStyle {
                            font: assets.load("fonts/FiraSans-SemiBold.ttf"), 
                            font_size: 30.0,  
                            color: Color::WHITE,  
                        },
                    },
                ],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(125.0),  
                top: Val::Px(300.0),
                ..default()
            },
            ..default()
        });
    })
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "User".to_string(),
                        style: TextStyle {
                            font: assets.load("fonts/FiraSans-SemiBold.ttf"), 
                            font_size: 30.0,  
                            color: Color::WHITE,  
                        },
                    },
                ],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(15.0),  
                top: Val::Px(15.0),
                ..default()
            },
            ..default()
        });
    })
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Balance: x".to_string(),
                        style: TextStyle {
                            font: assets.load("fonts/FiraSans-SemiBold.ttf"), 
                            font_size: 30.0,  
                            color: Color::WHITE,  
                        },
                    },
                ],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(150.0),  
                top: Val::Px(15.0),
                ..default()
            },
            ..default()
        });
    })
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Dealer".to_string(),
                        style: TextStyle {
                            font: assets.load("fonts/FiraSans-SemiBold.ttf"), 
                            font_size: 30.0,  
                            color: Color::WHITE,  
                        },
                    },
                ],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(415.0),  
                top: Val::Px(15.0),
                ..default()
            },
            ..default()
        });
    })
    .with_children(|parent| {
        
        parent.spawn(ImageBundle {
            style: Style {
                width: Val::Px(150.0), 
                height: Val::Px(195.0),
                position_type: PositionType::Absolute,
                left: Val::Px(450.0),  
                top: Val::Px(100.0),   
                ..default()
            },
            image: UiImage {
                texture: assets.load("deck/card_back.png"),  
                ..default()
            },
            ..default()
        });
    })
    .with_children(|parent| {
        
        parent.spawn(ImageBundle {
            style: Style {
                width: Val::Px(150.0), 
                height: Val::Px(195.0),
                position_type: PositionType::Absolute,
                left: Val::Px(605.0),  
                top: Val::Px(100.0),   
                ..default()
            },
            image: UiImage {
                texture: assets.load("deck/card_back.png"),  
                ..default()
            },
            ..default()
        });
    })
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(90.0),
                height: Val::Px(50.0),
                position_type: PositionType::Absolute, 
                left: Val::Px(690.0),   
                top: Val::Px(10.0),  
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect {left: Val::Px(10.0), bottom: Val::Px(10.0), ..default()},
                ..default()

            },
            border_color: BorderColor(Color::BLACK),
            background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Home",
             TextStyle {
                font: assets.load("fonts/FiraSans-SemiBold.ttf"),
                font_size: 15.0,
                color: Color::srgb(0.9, 0.9, 0.9),
             }
            ));
        });
    });
    
}