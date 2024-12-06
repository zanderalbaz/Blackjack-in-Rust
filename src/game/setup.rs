
use bevy::prelude::*;
use crate::game::components::{Background, Logo, PressEnterToPlay};

pub fn start_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut texture_atlas: ResMut<Assets<TextureAtlasLayout>>)
    {
        commands.spawn(Camera2dBundle::default());
        commands.spawn((
            SpriteBundle{
                texture: asset_server.load("background.png"), //automatically looks in assets folder
                sprite: Sprite {
                    custom_size: Some(Vec2::new(800., 500.)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0), //this determines the position of the sprite
                ..default()
            },
            Background,
        ));
        commands.spawn((
            SpriteBundle{
                texture: asset_server.load("game_title.png"), //automatically looks in assets folder
                sprite: Sprite {
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 1.0), //this determines the position of the sprite
                ..default()
            },
            Logo,
        ));
        commands.spawn((
            SpriteBundle{
                texture: asset_server.load("press_enter_to_play.png"), //automatically looks in assets folder
                sprite: Sprite {
                    custom_size: Some(Vec2::new(400., 50.)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, -150.0, 2.0), //this determines the position of the sprite
                ..default()
            },
            PressEnterToPlay,
        ));
        println!("Setup function ran");

}

pub fn setupScreen_setup(mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut texture_atlas: ResMut<Assets<TextureAtlasLayout>>,
    query: Query<Entity, With<Logo>>, 
    query2: Query<Entity, With<PressEnterToPlay>>) {

        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in query2.iter() {
            commands.entity(entity).despawn();
        }

        commands.spawn((
            SpriteBundle{
                texture: asset_server.load("background.png"), 
                sprite: Sprite {
                    custom_size: Some(Vec2::new(800., 500.)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0), 
                ..default()
            },
            Background,
        ));

    }