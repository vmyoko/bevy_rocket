use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowResolution},
};

mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Rocket"),
                resolution: WindowResolution::new(1280., 720.),
                resizable: false,
                enabled_buttons: EnabledButtons {
                    maximize: false,
                    minimize: false,
                    close: true,
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugins(game::GamePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: asset_server.load("landscape.png"),
        ..Default::default()
    });
}
