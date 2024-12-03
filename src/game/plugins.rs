use bevy::prelude::*;
pub struct SetupPlugin;

impl Plugin for SetupPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window{
                title: "Blackjack In Rust".to_string(),
                resolution: (800.,500.).into(),
                resizable: false,
                ..default() //sets all other params to defaults 
            }),
            ..default()
        }));
        println!("build setup function ran!");
    }
}