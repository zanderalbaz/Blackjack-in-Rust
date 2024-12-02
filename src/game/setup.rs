
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut texture_atlas: ResMut<Assets<TextureAtlasLayout>>)
    {
        commands.spawn(Camera2dBundle::default());
        println!("Setup function ran");

}