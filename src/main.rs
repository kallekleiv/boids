mod boid;
mod constants;

use boid::Boid;
use constants::NUM_BOIDS;

fn main() {
    let boids = vec![Boid::new(); NUM_BOIDS];
    for mut boid in boids {
        let neighbors = boid.get_neighbors(&boids);
        if !neighbors.is_empty() {
            boid.separation(&neighbors);
            boid.cohesion(&neighbors);
            boid.alignment(&neighbors);
        }
        boid.update_position();
    }
}
