pub mod components;
pub mod systems;
pub mod plugins;
pub mod setup;

use bevy::prelude::*;
use plugins::SetupPlugin;
use setup::setup;


pub fn run(){
	//start game here.
	App::new()
	.add_systems(Startup, setup)
	.add_plugins(SetupPlugin)
	.run();
	println!("Application ran!");
}
