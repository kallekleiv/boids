mod constants;

use constants::VIEW_RADIUS;

#[derive(Clone)]
pub struct Boid {
    x: f32,
    y: f32,
    v_x: f32,
    v_y: f32,
}

impl Boid {
    pub fn new() -> Boid {
        Boid {
            x: 0.0,
            y: 0.0,
            v_x: 0.0,
            v_y: 0.0,
        }
    }

    pub fn update_position(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn update_velocity(&mut self, d_v_x: f32, d_v_y: f32) {
        self.v_x += d_v_x;
        self.v_y += d_v_y;
    }

    pub fn distance_to(&self, neighbor: &Boid) -> f32 {
        let dx: f32 = neighbor.x - self.x;
        let dy: f32 = neighbor.y - self.y;
        return dx * dx + dy * dy;
    }

    pub fn is_visible(&self, neighbor: &Boid) -> bool {
        self.distance_to(neighbor) < VIEW_RADIUS
    }

    pub fn separation(&self, neighbors: &[Boid]) -> (f32, f32) {
        return (0.0, 0.0);
    }

    pub fn alignment(&self, neighbors: &[Boid]) -> (f32, f32) {
        return (0.0, 0.0);
    }

    pub fn cohesion(&self, neighbors: &[Boid]) -> (f32, f32) {
        return (0.0, 0.0);
    }
}
