use bevy::prelude::*;
use super::{components::PressEnterToPlay, constants::AppState};


pub fn start_game(mut query: Query<&mut Transform, With<PressEnterToPlay>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,

){
    if keyboard_input.just_pressed(KeyCode::Enter) {

        next_state.set(AppState::Setup);

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