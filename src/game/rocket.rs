use bevy::prelude::*;

const ROCKET_SCALE: f32 = 0.5;

pub struct RocketPlugin;

impl Plugin for RocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, boost.run_if(has_boost_input));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Rocket::default(),
        SpriteBundle {
            texture: asset_server.load("cohete_off.png"),
            transform: Transform::from_xyz(0., 0., 3.).with_scale(Vec3::new(
                ROCKET_SCALE,
                ROCKET_SCALE,
                ROCKET_SCALE,
            )),
            ..Default::default()
        },
    ));
}

fn boost(mut rocket: Query<&mut Rocket>) {
    for mut rocket in &mut rocket {
        rocket.state = RocktState::Boosting;
    }
}

fn has_boost_input(keyboard_input: Res<Input<KeyCode>>) -> bool {
    keyboard_input.pressed(KeyCode::Space)
        || keyboard_input.pressed(KeyCode::Up)
        || keyboard_input.pressed(KeyCode::Down)
}

#[derive(Default)]
enum RocktState {
    #[default]
    Grounded,
    Boosting,
}

#[derive(Component, Default)]
pub struct Rocket {
    state: RocktState,
}
