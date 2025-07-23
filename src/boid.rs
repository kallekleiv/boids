use crate::constants::{AVOID_FACTOR, CENTERING_FACTOR, MATCHING_FACTOR, VIEW_RADIUS};

#[derive(Clone)]
pub struct Boid {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

impl Boid {
    pub fn new() -> Boid {
        Boid {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
        }
    }

    pub fn update_position(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }

    pub fn update_velocity(&mut self, d_vx: f32, d_vy: f32) {
        self.vx += d_vx;
        self.vy += d_vy;
    }

    pub fn distance_to(&self, neighbor: &Boid) -> f32 {
        let dx: f32 = neighbor.x - self.x;
        let dy: f32 = neighbor.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn is_visible(&self, neighbor: &Boid) -> bool {
        self.distance_to(neighbor) < VIEW_RADIUS
    }

    pub fn get_neighbors(&self, boids: &Vec<Boid>) -> &Vec<Boid> {
        let mut neighbors = Vec::new();
        for boid in boids {
            if self.is_visible(boid) {
                neighbors.push(boid.clone())
            }
        }
        neighbors
    }

    // Each boid attempts to avoid running into other boids. If two or more boids get too close to one another (i.e. within one another's protected range), they will steer away from one another. They will do so in the following way:
    pub fn separation(&mut self, neighbors: &[Boid]) {
        let mut close_dx: f32 = 0.0;
        let mut close_dy: f32 = 0.0;

        for neighbor in neighbors {
            close_dx += self.x - neighbor.x;
            close_dy += self.y - neighbor.y;
        }

        self.update_velocity(close_dx * AVOID_FACTOR, close_dy * AVOID_FACTOR);
    }

    //Each boid attempts to match the velocity of other boids inside its visible range. It does so in the following way:
    pub fn alignment(&mut self, neighbors: &[Boid]) {
        let mut xvel_avg: f32 = 0.0;
        let mut yvel_avg: f32 = 0.0;
        let neighboring_boids: f32 = neighbors.len() as f32;

        for neighbor in neighbors {
            xvel_avg += neighbor.vx;
            yvel_avg += neighbor.vy;
        }

        xvel_avg = xvel_avg / neighboring_boids;
        yvel_avg = yvel_avg / neighboring_boids;

        self.update_velocity(
            (xvel_avg - self.vx) * MATCHING_FACTOR,
            (yvel_avg - self.vy) * MATCHING_FACTOR,
        );
    }

    // Each boid steers gently toward the center of mass of other boids within its visible range. It does so in the following way:
    pub fn cohesion(&mut self, neighbors: &[Boid]) {
        let mut xpos_avg: f32 = 0.0;
        let mut ypos_avg: f32 = 0.0;
        let neighboring_boids: f32 = neighbors.len() as f32;

        for neighbor in neighbors {
            xpos_avg += neighbor.x;
            ypos_avg += neighbor.y;
        }

        xpos_avg = xpos_avg / neighboring_boids;
        ypos_avg = ypos_avg / neighboring_boids;

        self.update_velocity(
            (xpos_avg - self.x) * CENTERING_FACTOR,
            (ypos_avg - self.y) * CENTERING_FACTOR,
        );
    }
}
