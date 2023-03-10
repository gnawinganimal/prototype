use bevy::prelude::*;

// currently path is simply collection of straight lines

// also note - paths must contain at least two points to be valid

pub struct Bezier {
    curves: Vec<[Vec2; 4]>,
}

impl Bezier {
    pub fn new(curves: impl Into<Vec<[Vec2; 4]>>) -> Self {
        Self {
            curves: curves.into(),
        }
    }

    pub fn len(&self) -> f32 {
        self.curves.len() as f32
    }

    pub fn get(&self, u: f32) -> Option<Vec2> {
        self.get_split(u.floor() as usize, u - u.floor())
    }

    pub fn get_split(&self, i: usize, t: f32) -> Option<Vec2> {
        if i == self.curves.len() {
            return Some(self.curves.last()?[3])
        }

        let [p0, p1, p2, p3] = self.curves.get(i).copied()?;

        Some(((1.0 - t).powi(3) * p0)
        + (3.0 * (1.0 - t).powi(2) * t * p1)
        + (3.0 * (1.0 - t) * t.powi(2) * p2)
        + (t.powi(3) * p3))
    }

    pub fn get_curve(&self, u: f32) -> Option<[Vec2; 4]> {
        self.curves.get(u.floor() as usize).copied()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Bez3(pub Vec2, pub Vec2, pub Vec2, pub Vec2);

impl Bez3 {
    pub fn first(&self) -> Vec2 {
        self.0
    }

    pub fn last(&self) -> Vec2 {
        self.3
    }

    pub fn get(&self, t: f32) -> Option<Vec2> {
        if t < 0.0 || t > 1.0 {
            return None;
        }

        Some(((1.0 - t).powi(3) * self.0)
            + (3.0 * (1.0 - t).powi(2) * t * self.1)
            + (3.0 * (1.0 - t) * t.powi(2) * self.2)
            + (t.powi(3) * self.3))
    }
}
