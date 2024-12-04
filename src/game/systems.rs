use bevy::prelude::*;
use super::{components::{PlayerName, PressEnterToPlay, Balance, Hands, Card, Hand}, constants::AppState};
use super::bundles::PlayerBundle;

pub fn start_game(mut query: Query<&mut Transform, With<PressEnterToPlay>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,

){
    if keyboard_input.just_pressed(KeyCode::Enter) {

        next_state.set(AppState::Setup);

    }
}

pub fn spawn_test_player(mut commands: Commands){
    commands.spawn(PlayerBundle{
        player_name: PlayerName(String::from("test").into()),
        balance: Balance(100.),
        hands: Hands(vec![Hand{
            cards: //This is good for a test, but later we should spawn card components
            (Card{ 
                suite: String::from("spade").into(), 
                face: String::from("2").into(), 
                value: (3,0) 
            }, 
            Card{
                suite: String::from("spade").into(),
                face: String::from("3").into(),
                value: (3,0)
            })
        }]),
    });
}

pub fn test_player_balance_change(mut query: Query<&mut Balance>){
    for mut balance in &mut query{
        println!("Player has balance of {}", balance.0);
        balance.0 += 1.;
        println!("Player has updated balance of {}", balance.0);
    }
}

pub fn setup_screen(mut commands: Commands, assets: Res<AssetServer>) {
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
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                position_type: PositionType::Absolute, 
                left: Val::Px(315.0),   
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
            parent.spawn(TextBundle::from_section("Play",
             TextStyle {
                font: assets.load("fonts/FiraSans-SemiBold.ttf"),
                font_size: 40.0,
                color: Color::srgb(0.9, 0.9, 0.9),
             }
            ));
        });
    });
}