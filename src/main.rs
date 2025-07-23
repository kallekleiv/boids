mod boid;
mod constants;

use boid::Boid;
use constants::{NUM_BOIDS, VIEW_RADIUS};

fn main() {
    let boids = vec![Boid::new(); NUM_BOIDS];
    for boid in boids.iter() {
        let neighbors = get_neighbors(&boid);
        boid.separation(neighbors);
        boid.cohesion(neighbors);
        boid.alignment(neighbors);
    }
}

fn get_neighbors(boid: &Boid) {}
