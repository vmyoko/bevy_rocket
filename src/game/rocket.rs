use bevy::prelude::*;

const ROCKET_SCALE: f32 = 0.5;

pub struct RocketPlugin;

impl Plugin for RocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Component, Default)]
pub struct Rocket {}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Rocket::default(),
        SpriteBundle {
            texture: asset_server.load("cohete_off.png"),
            transform: Transform::from_xyz(0., 0., 3.).with_scale(Vec3::new(ROCKET_SCALE, ROCKET_SCALE, ROCKET_SCALE)),
            ..Default::default()
        },
    ));
}
