struct Boid {
    x: f32,
    y: f32,
    v_x: f32,
    v_y: f32,
}

impl Boid {
    fn new() -> Boid {
        Boid {
            x: 0,
            y: 0,
            v_x: 0,
            v_y: 0,
        }
    }

    fn update_position(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }

    fn update_velocity(&mut self, d_v_x: f32, d_v_y: f32) {
        self.v_x += d_v_x;
        self.v_y += d_v_y;
    }
}
