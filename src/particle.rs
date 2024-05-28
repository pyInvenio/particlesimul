
use ultraviolet::Vec3;
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Body {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub mass: f32,
    pub density: f32,
}

pub trait Particle {

    fn position(&self) -> Vec3;
    fn velocity(&self) -> Vec3;
    fn acceleration(&self) -> Vec3;
    fn mass(&self) -> f32;
    fn density(&self) -> f32;

    fn set_position(&mut self, position: Vec3);
    fn set_velocity(&mut self, velocity: Vec3);
    fn set_acceleration(&mut self, acceleration: Vec3);

    fn new_position(&mut self, dt: f32);
    fn new_velocity(&mut self, dt: f32);

}

impl Particle for Body {
    fn position(&self) -> Vec3 {
        self.position
    }

    fn velocity(&self) -> Vec3 {
        self.velocity
    }

    fn acceleration(&self) -> Vec3 {
        self.acceleration
    }

    fn mass(&self) -> f32 {
        self.mass
    }

    fn density(&self) -> f32 {
        self.density
    }

    fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    fn set_velocity(&mut self, velocity: Vec3) {
        self.velocity = velocity;
    }

    fn set_acceleration(&mut self, acceleration: Vec3) {
        self.acceleration = acceleration;
    }

    fn new_position(&mut self, dt: f32) {
        self.set_position(self.position + self.velocity * dt);
    }

    fn new_velocity(&mut self, dt: f32) {
        self.set_velocity(self.velocity + self.acceleration * dt);
    }
}
