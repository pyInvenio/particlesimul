use ultraviolet::Vec3;
use crate::particle::{Body, Particle};

const G: f32 = 6.67430e-11;

pub trait Compute {
    fn calculate_force(&self, p1: &dyn Particle, p2: &dyn Particle) -> Vec3;
    fn update_position(&self, p: &mut dyn Particle, dt: f32);
    fn calculate_acceleration(&self, p: &dyn Particle, force: Vec3) -> Vec3;
    fn update_velocity(&self, p: &mut dyn Particle, dt: f32);
    fn simulate(&self, bodies: &mut Vec<Body>, dt: f32);
}

// impl compute for the Body struct
impl Compute for Body {
    fn calculate_force(&self, p1: &dyn Particle, p2: &dyn Particle) -> Vec3 {
        let r = p2.position() - p1.position();
        let r_mag = r.mag();
        let mut r_hat = r.clone();
        r_hat.normalize();
        let force_mag = G * p1.mass() * p2.mass() / (r_mag * r_mag);
        r_hat * force_mag
    }

    fn update_position(&self, p: &mut dyn Particle, dt: f32) {
        let new_position = p.position() + p.velocity() * dt;
        p.set_position(new_position);
    }

    fn calculate_acceleration(&self, p: &dyn Particle, force: Vec3) -> Vec3 {
        force / p.mass()
    }

    fn update_velocity(&self, p: &mut dyn Particle, dt: f32) {
        let new_velocity = p.velocity() + p.acceleration() * dt;
        p.set_velocity(new_velocity);
    }

    fn simulate(&self, bodies: &mut Vec<Body>, dt: f32) {
        let mut forces = vec![Vec3::zero(); bodies.len()];

        for i in 0..bodies.len() {
            for j in 0..bodies.len() {
                if i != j {
                    forces[i] += self.calculate_force(&bodies[i], &bodies[j]);
                }
            }
        }

        for i in 0..bodies.len() {
            let acceleration = self.calculate_acceleration(&bodies[i], forces[i]);
            bodies[i].set_acceleration(acceleration);
            self.update_velocity(&mut bodies[i], dt);
            self.update_position(&mut bodies[i], dt); // Change the parameter type to &mut Body
        }
    }
}