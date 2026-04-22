use std::ops::{Add, Sub};

#[derive(PartialEq, Clone, Copy)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32,
}

impl Vec2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Default for Vec2D {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl std::fmt::Display for Vec2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl Add<&Vec2D> for &Vec2D {
    type Output = Vec2D;

    fn add(self, other: &Vec2D) -> Vec2D {
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for Vec2D {
    type Output = Vec2D;

    fn add(self, other: Vec2D) -> Self {
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2D {
    type Output = Self;

    fn sub(self, other: Vec2D) -> Self {
        Vec2D {
            x: (self.x - other.x),
            y: (self.y - other.y),
        }
    }
}
