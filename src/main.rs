use bevy::prelude::*;

mod particle;
mod compute;

use particle::Body;
use compute::Compute;

use ultraviolet::Vec3;

fn setup(mut commands: Commands) {
    commands.spawn(Body {
        position: Vec3::new(0.0, 0.0, 0.0),
        velocity: Vec3::new(0.0, 0.0, 0.0),
        acceleration: Vec3::new(0.0, 0.0, 0.0),
        mass: 1.0e24,
    });
    commands.spawn(Body {
        position: Vec3::new(1.0e11, 0.0, 0.0),
        velocity: Vec3::new(0.0, 1.0e3, 0.0),
        acceleration: Vec3::new(0.0, 0.0, 0.0),
        mass: 1.0e24,
    });
    commands.spawn(Body {
        position: Vec3::new(-1.0e11, 0.0, 0.0),
        velocity: Vec3::new(0.0, -1.0e3, 0.0),
        acceleration: Vec3::new(0.0, 0.0, 0.0),
        mass: 1.0e24,
    });
}

fn simulate_system(mut query: Query<&mut Body>, time: Res<Time>) {
    let dt = time.delta_seconds_f64();

    let mut bodies: Vec<Body> = query.iter_mut().map(|body| body.clone()).collect();
    Body.simulate(&mut bodies, dt);

    for (i, mut body) in query.iter_mut().enumerate() {
        *body = bodies[i].clone();
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(simulate_system.system())
        .run();
}
