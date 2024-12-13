
use bevy::prelude::*;
use crate::game::components::{Background, Logo, PressEnterToPlay};

use super::components::{ChipButtonValue, InGameCardAccess, PlayerButtonValues, TextComponents};

pub fn start_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    )
    {
        commands.spawn(Camera2dBundle::default());
        spawn_home_assets(commands, asset_server);
}

pub fn ingame_screen_setup(mut commands: Commands, 
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

    pub fn reload_home_screen( mut commands: Commands, 
        asset_server: Res<AssetServer>, 
        mut in_game_query: ParamSet<(
            Query<Entity, With<PlayerButtonValues>>,
            Query<Entity, With<InGameCardAccess>>,
            Query<Entity, With<ChipButtonValue>>,
            Query<Entity, With<TextComponents>>,
        )>,
    ) {

        for entity in in_game_query.p0().iter() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in in_game_query.p1().iter() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in in_game_query.p2().iter() {
            commands.entity(entity).despawn_recursive();    
        }
        for entity in in_game_query.p3().iter() {
            commands.entity(entity).despawn_recursive();
        }

        spawn_home_assets(commands, asset_server);

    }

    pub fn spawn_home_assets( mut commands: Commands, 
        asset_server: Res<AssetServer>,

    ) {
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
        commands.spawn((
            SpriteBundle{
                texture: asset_server.load("game_title.png"), 
                sprite: Sprite {
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 1.0), 
                ..default()
            },
            Logo,
        ));
        commands.spawn((
            SpriteBundle{
                texture: asset_server.load("press_enter_to_play.png"), 
                sprite: Sprite {
                    custom_size: Some(Vec2::new(400., 50.)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, -150.0, 2.0), 
                ..default()
            },
            PressEnterToPlay,
        ));

    }