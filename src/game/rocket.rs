use bevy::prelude::*;

pub struct RocketPlugin;

impl Plugin for RocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hello);
    }
}

fn hello() {
    println!("Hello");
}