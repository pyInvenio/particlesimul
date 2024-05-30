use bevy::prelude::*;
use particlesimul::scenedata::create_scene;
use ultraviolet::Vec3;

mod compute;
mod particle;

use compute::Compute;
use particle::Body;
use particlesimul::scenedata::*;

fn main() {
    let mut scene = create_scene(SceneType::DefaultScene, None, None, None);

    // simulate and print out the positions of the particles
    for _ in 0..100 {
        render_and_simulate(&mut scene, 0.1);
    }

}
