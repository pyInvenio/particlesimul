use crate::particle::{Body, Particle};
use ultraviolet::Vec3;
use bevy::ecs::system::{EntityCommands, Res};
use bevy::prelude::*;
use rand::prelude::*;

use crate::compute::Compute;

const G: f32 = 6.67430e-11;

#[derive(Bundle)]
pub struct BodyBundle {
    pub body: Body,
    pub transform: Transform,
    pub sprite: SpriteBundle,
}

impl BodyBundle {
    pub fn new(body: Body) -> Self {
        let bevy_vec3: bevy::prelude::Vec3 =
            bevy::prelude::Vec3::new(body.position.x, body.position.y, body.position.z);

        let random_color = Color::rgb(
            rand::random::<f32>(),
            rand::random::<f32>(),
            rand::random::<f32>(),
        );
        BodyBundle {
            body,
            transform: Transform::from_translation(bevy_vec3),
            sprite: SpriteBundle {
                transform: Transform::from_translation(bevy_vec3),
                sprite: Sprite {
                    color: random_color,
                    custom_size: Some(Vec2::new(5.0, 5.0)),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub enum SceneType {
    CustomScene,
    DefaultScene,
    FigureEight,
    SolarSystem,
}

#[derive(Resource)]
pub struct Scene {
    pub scene_type: SceneType,
    pub bodies_count: usize,
    pub bodies: Vec<Body>,
    pub bodies_min_mass: f32,
    pub bodies_max_mass: f32,
    pub compute: Compute,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            scene_type: SceneType::DefaultScene,
            bodies_count: 0,
            bodies: Vec::new(),
            bodies_min_mass: 0.0,
            bodies_max_mass: 0.0,
            compute: Compute {},
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn add_bodies(&mut self, bodies: Vec<Body>) {
        self.bodies.extend(bodies);
    }

    pub fn add_bodies_bundle(&mut self, commands: &mut Commands) {
        for i in 0..self.bodies.len() {
            commands.spawn(BodyBundle::new(self.bodies[i].clone()));
        }
    }
}

pub struct DefaultScene;
pub struct FigureEight;
pub struct SolarSystem;
pub struct CustomScene;

impl DefaultScene {
    pub fn new() -> Scene {
        let mut bodies = Vec::new();
        let sun = Body {
            position: Vec3::zero(),
            velocity: Vec3::zero(),
            acceleration: Vec3::zero(),
            mass: 1.0, // Arbitrary mass for the Sun
            density: 1400.0,
        };
        let earth = Body {
            position: Vec3::new(1.0, 0.0, 0.0), // Arbitrary distance from the Sun
            velocity: Vec3::new(0.0, 1.0, 0.0), // Arbitrary velocity
            acceleration: Vec3::zero(),
            mass: 0.001, // Arbitrary mass for the Earth
            density: 5500.0,
        };
        let moon = Body {
            position: Vec3::new(1.2, 0.0, 0.0), // Arbitrary distance from the Earth
            velocity: Vec3::new(0.0, 1.2, 0.0), // Arbitrary velocity
            acceleration: Vec3::zero(),
            mass: 0.0001, // Arbitrary mass for the Moon
            density: 3300.0,
        };

        bodies.push(sun);
        bodies.push(earth);
        bodies.push(moon);

        let compute = Compute {};

        Scene {
            scene_type: SceneType::DefaultScene,
            bodies_count: bodies.len(),
            bodies,
            bodies_min_mass: 0.0001,
            bodies_max_mass: 1.0,
            compute,
        }
    }
}

impl CustomScene {
    pub fn new(count: usize, min_mass: f32, max_mass: f32) -> Scene {
        let mut bodies = Vec::new();
        for _ in 0..count {
            let body = Body {
                position: Vec3::zero(),
                velocity: Vec3::zero(),
                acceleration: Vec3::zero(),
                mass: min_mass + (max_mass - min_mass) * rand::random::<f32>(),
                density: 1410.0,
            };
            bodies.push(body);
        }
        let compute = Compute {};
        Scene {
            scene_type: SceneType::CustomScene,
            bodies_count: count,
            bodies,
            bodies_min_mass: min_mass,
            bodies_max_mass: max_mass,
            compute,
        }
    }
}

impl FigureEight {
    pub fn new() -> Scene {
        let mut bodies = Vec::new();
        let sun = Body {
            position: Vec3::zero(),
            velocity: Vec3::zero(),
            acceleration: Vec3::zero(),
            mass: 1.989e30,
            density: 1410.0,
        };
        let earth = Body {
            position: Vec3::new(1.496e11, 0.0, 0.0),
            velocity: Vec3::new(0.0, 29783.0, 0.0),
            acceleration: Vec3::zero(),
            mass: 5.972e24,
            density: 5515.0,
        };
        let moon = Body {
            position: Vec3::new(1.496e11 + 3.844e8, 0.0, 0.0),
            velocity: Vec3::new(0.0, 29783.0 + 1022.0, 0.0),
            acceleration: Vec3::zero(),
            mass: 7.342e22,
            density: 3344.0,
        };
        bodies.push(sun);
        bodies.push(earth);
        bodies.push(moon);
        let compute = Compute {};
        Scene {
            scene_type: SceneType::FigureEight,
            bodies_count: 3,
            bodies,
            bodies_min_mass: 1.0,
            bodies_max_mass: 5.0,
            compute,
        }
    }
}

impl SolarSystem {
    pub fn new() -> Scene {
        let mut bodies = Vec::new();
        let sun = Body {
            position: Vec3::zero(),
            velocity: Vec3::zero(),
            acceleration: Vec3::zero(),
            mass: 1.989e30,
            density: 1410.0,
        };
        let earth = Body {
            position: Vec3::new(1.496e11, 0.0, 0.0),
            velocity: Vec3::new(0.0, 29783.0, 0.0),
            acceleration: Vec3::zero(),
            mass: 5.972e24,
            density: 5515.0,
        };
        let moon = Body {
            position: Vec3::new(1.496e11 + 3.844e8, 0.0, 0.0),
            velocity: Vec3::new(0.0, 29783.0 + 1022.0, 0.0),
            acceleration: Vec3::zero(),
            mass: 7.342e22,
            density: 3344.0,
        };
        bodies.push(sun);
        bodies.push(earth);
        bodies.push(moon);
        let compute = Compute {};
        Scene {
            scene_type: SceneType::SolarSystem,
            bodies_count: 3,
            bodies,
            bodies_min_mass: 1.0,
            bodies_max_mass: 5.0,
            compute,
        }
    }
}

pub fn create_scene(
    st: SceneType,
    bodies_count: Option<usize>,
    bodies_min_mass: Option<f32>,
    bodies_max_mass: Option<f32>,
) -> Scene {
    match st {
        SceneType::CustomScene => {
            let count = bodies_count.unwrap_or(3);
            let min_mass = bodies_min_mass.unwrap_or(1.0);
            let max_mass = bodies_max_mass.unwrap_or(5.0);
            CustomScene::new(count, min_mass, max_mass)
        }

        SceneType::DefaultScene => DefaultScene::new(),
        SceneType::FigureEight => FigureEight::new(),
        SceneType::SolarSystem => SolarSystem::new(),
    }
}

pub fn render_and_simulate(scene: &mut Scene, dt: f32) {
    scene.compute.simulate(dt, G, &mut scene.bodies);

    for i in 0..scene.bodies.len() {
        let body = &scene.bodies[i];
        println!("Body {} position: {:?}", i, body.position);
    }
}
