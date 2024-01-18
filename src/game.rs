use bevy::prelude::*;

mod rocket;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hello);
    }
}

fn hello() {
    println!("Hello");
}