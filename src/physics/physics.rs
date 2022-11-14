#[derive(Copy, Clone)]
pub struct Vector2 {
    v: [f32; 2],
}

#[allow(dead_code)]
impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { v: [x, y] }
    }

    pub fn x(&self) -> f32 {
        self.v[0]
    }

    pub fn y(&self) -> f32 {
        self.v[1]
    }

    pub fn up() -> Vector2 {
        Vector2 { v: [0.0, -1.0] }
    }

    pub fn down() -> Vector2 {
        Vector2 { v: [0.0, 1.0] }
    }

    pub fn left() -> Vector2 {
        Vector2 { v: [-1.0, 0.0] }
    }

    pub fn right() -> Vector2 {
        Vector2 { v: [1.0, 0.0] }
    }

    pub fn set_x(&mut self, x: f32) {
        self.v[0] = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.v[1] = y;
    }

    pub fn add(&self, other: Vector2) -> Vector2 {
        Vector2 {
            v: [self.v[0] + other.v[0], self.v[1] + other.v[1]],
        }
    }

    pub fn sub(&self, other: Vector2) -> Vector2 {
        Vector2 {
            v: [self.v[0] - other.v[0], self.v[1] - other.v[1]],
        }
    }

    pub fn mul(&self, other: Vector2) -> Vector2 {
        Vector2 {
            v: [self.v[0] * other.v[0], self.v[1] * other.v[1]],
        }
    }

    pub fn div(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            v: [self.v[0] / other.v[0], self.v[1] / other.v[1]],
        }
    }

    pub fn add_scalar(&self, scalar: f32) -> Vector2 {
        Vector2 {
            v: [self.v[0] + scalar, self.v[1] + scalar],
        }
    }
}
