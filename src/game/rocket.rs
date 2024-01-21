use bevy::prelude::*;

const ROCKET_SCALE: f32 = 0.5;
const ROCKET_MAX_SPEED: f32 = 150.;
const ROCKET_ACCELERATION: f32 = 2.;
const ROCKET_DRAG: f32 = 1.;
const GRAVITY: f32 = 1.;
const TERMINAL_VELOCITY: f32 = 300.;
const TURNING_SPEED: f32 = 90. * 0.01745; //degrees multiplied by PI/180 to convert to radians

pub struct RocketPlugin;

impl Plugin for RocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (rotate, boost, fall, update));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Rocket {
            direction: Vec3::new(0., 1., 0.),
            ..default()
        },
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

fn rotate(mut query: Query<&mut Rocket>, keyboard_input: Res<Input<KeyCode>>, time: Res<Time>) {
    for mut rocket in &mut query {
        if !keyboard_input.pressed(KeyCode::Left) && !keyboard_input.pressed(KeyCode::Right) {
            rocket.rotate = 0.;
            break;
        }
        if rocket.state == RocketState::Grounded {
            break;
        }
        let turn_direction = if keyboard_input.pressed(KeyCode::Left) {
            1.
        } else {
            -1.
        };
        rocket.rotate = turn_direction * TURNING_SPEED * time.delta_seconds();
    }
}

fn boost(mut rocket: Query<(&mut Rocket, &mut Transform)>, keyboard_input: Res<Input<KeyCode>>) {
    let has_boost_input = keyboard_input.pressed(KeyCode::Space)
        || keyboard_input.pressed(KeyCode::Up)
        || keyboard_input.pressed(KeyCode::Down);

    for (mut rocket, mut transform) in &mut rocket {
        transform.rotate_z(rocket.rotate);
        let rotation_quat = Quat::from_rotation_z(rocket.rotate);
        let rotation = rotation_quat
            * Quat::from_vec4(Vec4::new(
                rocket.direction.x,
                rocket.direction.y,
                rocket.direction.z,
                0.,
            ))
            * rotation_quat.inverse();
        let rotation_vec3 = Vec3::new(rotation.x, rotation.y, rotation.z);
        rocket.direction = rotation_vec3.normalize();

        if has_boost_input {
            rocket.state = RocketState::Boosting;
            let speed = f32::min(
                ROCKET_MAX_SPEED,
                rocket.velocity.length() + ROCKET_ACCELERATION,
            );
            rocket.velocity = rocket.direction * speed;
        } else if rocket.state != RocketState::Grounded && rocket.state != RocketState::Falling {
            let speed = f32::max(0., rocket.velocity.length() - ROCKET_DRAG);
            rocket.state = if speed > 0. {
                RocketState::Inert
            } else {
                RocketState::Falling
            };
            rocket.velocity = rocket.direction * speed;
        }
    }
}

fn fall(mut rocket: Query<&mut Rocket>) {
    for mut rocket in &mut rocket {
        if rocket.state == RocketState::Falling {
            let velocity = if rocket.velocity.length() < TERMINAL_VELOCITY {
                rocket.velocity - Vec3::new(0., GRAVITY, 0.)
            } else {
                Vec3::new(0., -TERMINAL_VELOCITY, 0.)
            };
            rocket.velocity = velocity;
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
enum RocketState {
    #[default]
    Grounded,
    Boosting,
    Inert,
    Falling,
}

#[derive(Component, Default)]
pub struct Rocket {
    state: RocketState,
    velocity: Vec3,
    rotate: f32,
    direction: Vec3,
}
