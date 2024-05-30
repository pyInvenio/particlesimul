use ultraviolet::Vec3;
use crate::particle::{self, Body, Particle};


pub struct Compute {
    // vector of particles
    // pub particles : Vec<Body>,
}

// impl compute for the Body struct
impl Compute {
    pub fn calculate_force(&self, p1: &Body, p2: &Body, G:f32) -> Vec3 {
        // println!("p1: {:?}", p1.position());
        let r = p2.position() - p1.position();
        // println!("r: {:?}", r);
        let r_mag = r.mag();
        // println!("r_mag: {:?}", r_mag);
        let mut r_hat = r.clone();
        r_hat.normalize();
        // println!("r_hat: {:?}", r_hat);
        let force_mag = G * p1.mass() * p2.mass() / (r_mag * r_mag);
        // println!("force_mag: {:?}", force_mag);
        // println!("G: {:?}", G);
        // println!("{:?}", p1.mass());
        // println!("{:?}", p2.mass());
        // println!("{:?}", G * p1.mass() * p2.mass());
        // println!("{:?}", r_mag * r_mag);
        // panic!("");
        r_hat * force_mag
    }

    pub fn calculate_acceleration(&self, p: &Body, force: Vec3) -> Vec3 {
        force / p.mass()
    }

    pub fn simulate(&mut self, dt: f32, G:f32, particles: &mut Vec<Body>) {
        let mut forces = vec![Vec3::zero(); particles.len()];

        for i in 0..particles.len() {
            for j in 0..particles.len() {
                if i != j {
                    forces[i] += self.calculate_force(&particles[i], &particles[j], G);
                    // println!("Force: {:?}", forces[i])
                }
            }
        }

        for i in 0..particles.len() {
            let acceleration = self.calculate_acceleration(&particles[i], forces[i]);
            particles[i].set_acceleration(acceleration);
            particles[i].new_velocity(dt);
            particles[i].new_position(dt);
            // println!("Body {} position: {:?}", i, particles[i].position());
        }
    }
}