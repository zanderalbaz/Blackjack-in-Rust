
use bevy::prelude::*;
use crate::game::components::{Background, GameTitle};

pub fn setup(
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
            GameTitle,
        ));
        println!("Setup function ran");

}