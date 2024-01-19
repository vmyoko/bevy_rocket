use bevy::prelude::*;

const ROCKET_SCALE: f32 = 0.5;
const ROCKET_MAX_SPEED: f32 = 150.;
const ROCKET_ACCELERATION: f32 = 2.;
const ROCKET_DRAG: f32 = 1.;
const GRAVITY: f32 = 1.;
const TERMINAL_VELOCITY: f32 = 300.;

pub struct RocketPlugin;

impl Plugin for RocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (boost, fall, update));
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

fn boost(mut rocket: Query<&mut Rocket>, keyboard_input: Res<Input<KeyCode>>) {
    let has_boost_input = keyboard_input.pressed(KeyCode::Space)
        || keyboard_input.pressed(KeyCode::Up)
        || keyboard_input.pressed(KeyCode::Down);

    for mut rocket in &mut rocket {
        if has_boost_input {
            rocket.state = RocktState::Boosting;
            let speed = if rocket.velocity.length() < ROCKET_MAX_SPEED {
                rocket.velocity.length() + ROCKET_ACCELERATION
            } else {
                ROCKET_MAX_SPEED
            };
            rocket.velocity = Vec3::new(0., speed, 0.);
        } else if rocket.state != RocktState::Grounded && rocket.state != RocktState::Falling {
            rocket.state = RocktState::Inert;
            let speed = if rocket.velocity.length() > 0. {
                rocket.velocity.length() - ROCKET_DRAG
            } else {
                rocket.state = RocktState::Falling;
                0.
            };
            rocket.velocity = Vec3::new(0., speed, 0.);
        }
    }
}

fn fall(mut rocket: Query<&mut Rocket>) {
    for mut rocket in &mut rocket {
        if rocket.state == RocktState::Falling {
            let velocity = if rocket.velocity.length() < TERMINAL_VELOCITY {
                rocket.velocity - Vec3::new(0., GRAVITY, 0.)
            } else {
                Vec3::new(0., -TERMINAL_VELOCITY, 0.)
            };
            rocket.velocity = velocity;
            println!("{velocity}");
        }
    }
}

fn update(mut rocket: Query<(&Rocket, &mut Transform)>, time: Res<Time>) {
    for (rocket, mut transform) in &mut rocket {
        if rocket.velocity.length() != 0. {
            transform.translation += rocket.velocity * time.delta_seconds();
        }
    }
}

#[derive(Default, PartialEq)]
enum RocktState {
    #[default]
    Grounded,
    Boosting,
    Inert,
    Falling,
}

#[derive(Component, Default)]
pub struct Rocket {
    state: RocktState,
    velocity: Vec3,
}
