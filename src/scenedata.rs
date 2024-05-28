use crate::particle::{Body, Particle};
use ultraviolet::Vec3;

use bevy::ecs::system::{EntityCommands, Res};
use bevy::prelude::*;

const G: f32 = 6.67430e-11;

pub enum SceneType {
    CustomScene,
    DefaultScene,
    FigureEight,
    SolarSystem,
}

pub struct Scene {
    pub scene_type: SceneType,
    pub bodies_count: usize,
    pub bodies: Vec<Body>,
    pub bodies_min_mass: f32,
    pub bodies_max_mass: f32,
    pub compute: Compute,
}


pub struct DefaultScene {
    pub bodies: Vec<Body>,
}

impl DefaultScene {
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
        let compute = Compute { particles: bodies.clone() };
        Scene {
            scene_type: SceneType::DefaultScene,
            bodies_count: 3,
            bodies,
            bodies_min_mass: 1.0,
            bodies_max_mass: 5.0,
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
        let compute = Compute { particles: bodies.clone() };
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
        let compute = Compute { particles: bodies.clone() };
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
        let compute = Compute { particles: bodies.clone() };
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

impl CustomScene {}
pub fn create_scene(
    st: SceneType,
    bodies_count: Option<usize>,
    bodies_min_mass: Option<f32>,
    bodies_max_mass: Option<f32>,
) -> Scene {

    match st {
        SceneType::CustomScene => {
            let count = match bodies_count {
                Some(c) => c,
                None => 3,
            };
            let min_mass = match bodies_min_mass {
                Some(m) => m,
                None => 1.0,
            };
            let max_mass = match bodies_max_mass {
                Some(m) => m,
                None => 5.0,
            };
            CustomScene::new(count, min_mass, max_mass)
        }

        SceneType::DefaultScene => DefaultScene::new(),
        SceneType::FigureEight => FigureEight::new(),
        SceneType::SolarSystem => SolarSystem::new(),
    } {

    }
}

