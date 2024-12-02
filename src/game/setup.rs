
use bevy::prelude::*;
use crate::game::components::Background;

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
        println!("Setup function ran");

}