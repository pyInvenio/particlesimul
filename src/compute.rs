use ultraviolet::Vec3;
use crate::particle::{Body, Particle};


pub struct Compute {
    // vector of particles
    pub particles : Vec<Body>,
}

// impl compute for the Body struct
impl Compute {
    pub fn calculate_force(&self, p1: &Body, p2: &Body, G:f32) -> Vec3 {
        let r = p2.position() - p1.position();
        let r_mag = r.mag();
        let mut r_hat = r.clone();
        r_hat.normalize();
        let force_mag = G * p1.mass() * p2.mass() / (r_mag * r_mag);
        r_hat * force_mag
    }

    pub fn calculate_acceleration(&self, p: &Body, force: Vec3) -> Vec3 {
        force / p.mass()
    }

    pub fn simulate(&mut self, dt: f32, G:f32) {
        let mut forces = vec![Vec3::zero(); self.particles.len()];

        for i in 0..self.particles.len() {
            for j in 0..self.particles.len() {
                if i != j {
                    forces[i] += self.calculate_force(&self.particles[i], &self.particles[j], G);
                }
            }
        }

        for i in 0..self.particles.len() {
            let acceleration = self.calculate_acceleration(&self.particles[i], forces[i]);
            self.particles[i].set_acceleration(acceleration);
            self.particles[i].new_velocity(dt);
            self.particles[i].new_position(dt);
        }
    }
}