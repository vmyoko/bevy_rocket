use bevy::prelude::*;

use self::rocket::RocketPlugin;

mod rocket;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RocketPlugin);
    }
}

