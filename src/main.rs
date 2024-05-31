use bevy::prelude::*;
use particlesimul::scenedata::create_scene;
use ultraviolet::Vec3;

mod compute;
mod particle;
mod scenedata;

use bevy_pancam::{PanCam, PanCamPlugin};

use compute::Compute;
use particle::Body;
use particlesimul::scenedata::*;
use scenedata::Scene;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PanCamPlugin::default()))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (spawn_camera, setup))
        .add_systems(Update, simulate_and_render)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        PanCam {
            grab_buttons: vec![MouseButton::Right, MouseButton::Middle],
            ..default()
        },
    ));
}

fn setup(mut commands: Commands) {
    let mut scene = DefaultScene::new();
    scene.add_bodies_bundle(&mut commands);
    commands.insert_resource(scene);
}

#[derive(Component)]
struct BodyId(usize);

fn simulate_and_render(
    time: Res<Time>,
    mut scene: ResMut<Scene>,
    mut query: Query<(&BodyId, &mut Transform)>,
) {
    let dt = time.delta_seconds();
    let mut compute = Compute {};
    compute.simulate(dt, 6.67430e-11, &mut scene.bodies);

    for (body_id, mut transform) in query.iter_mut() {
        let body = &scene.bodies[body_id.0];
        transform.translation =
            bevy::prelude::Vec3::new(body.position.x, body.position.y, body.position.z);
        println!("Body {} position: {:?}", body_id.0, body.position);
    }
}
