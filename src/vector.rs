use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn repeat(value: f32) -> Self {
        Self {
            x: value,
            y: value
        }
    }

    pub fn rotate(self, angle: f32) -> Self {
        Self {
            x: (angle.cos() * self.x) - (angle.sin() * self.y),
            y: (angle.sin() * self.x) + (angle.cos() * self.y),
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
