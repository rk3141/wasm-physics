#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn add(&mut self, v2: Vector2) {
        self.x += v2.x;
        self.y += v2.y;
    }

    pub fn add_f64(&mut self, f: f64) {
        self.x += f;
        self.y += f;
    }

    pub fn mult(&mut self, v2: Vector2) {
        self.x *= v2.x;
        self.y *= v2.y;
    }

    pub fn mult_f64(&mut self, f: f64) {
        self.x *= f;
        self.y *= f;
    }

    pub fn dist(&self, other: &Vector2) -> f64 {
        ((self.x - other.x).powi(2) - (self.y - other.y).powi(2)).sqrt()
    }
}
