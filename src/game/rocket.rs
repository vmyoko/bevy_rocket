use bevy::prelude::*;

const ROCKET_SCALE: f32 = 0.5;
const ROCKET_MAX_SPEED: f32 = 300.;
const ROCKET_ACCELERATION: f32 = 2.;
const GRAVITY: f32 = 300.;
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
            force_compensation: 1.,
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

fn boost(
    mut rocket: Query<(&mut Rocket, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
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
            rocket.ballistic_trajectory = 0.;
            if rocket.force_compensation < 1. {
                rocket.velocity = rocket
                    .last_fall_velocity
                    .lerp(rocket.direction, rocket.force_compensation);
                rocket.force_compensation =
                    f32::min(1., rocket.force_compensation + time.delta_seconds());
            } else {
                rocket.state = RocketState::Boosting;
                let speed = f32::min(
                    ROCKET_MAX_SPEED,
                    rocket.velocity.length() + ROCKET_ACCELERATION,
                );
                rocket.velocity = rocket.direction * speed;
                rocket.last_boost_velocity = rocket.velocity;
            }
        } else if rocket.state != RocketState::Grounded {
            rocket.state = RocketState::Falling;
        }
    }
}

fn fall(mut rocket: Query<&mut Rocket>, time: Res<Time>) {
    let gravity_acceleration = Vec3::new(0., -GRAVITY, 0.);
    for mut rocket in &mut rocket {
        if rocket.state == RocketState::Falling {
            if rocket.ballistic_trajectory < 1. {
                rocket.velocity = rocket
                    .last_boost_velocity
                    .lerp(gravity_acceleration, rocket.ballistic_trajectory);
                rocket.ballistic_trajectory =
                    f32::min(1., rocket.ballistic_trajectory + time.delta_seconds());
            } else {
                rocket.force_compensation = 0.;
                let velocity = if rocket.velocity.length() < GRAVITY {
                    rocket.velocity - gravity_acceleration
                } else {
                    Vec3::new(0., -GRAVITY, 0.)
                };
                rocket.velocity = velocity;
                rocket.last_fall_velocity = velocity;
            }
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
    Falling,
}

#[derive(Component, Default)]
pub struct Rocket {
    state: RocketState,
    velocity: Vec3,
    rotate: f32,
    direction: Vec3,
    ballistic_trajectory: f32,
    last_fall_velocity: Vec3,
    force_compensation: f32,
    last_boost_velocity: Vec3,
}
