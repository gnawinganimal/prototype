use std::ops::Mul;

use bevy::prelude::*;

// currently path is simply collection of straight lines

// also note - paths must contain at least two points to be valid

pub struct Path {
    curves: Vec<Bez3>,
}

impl Path {
    pub fn new(slice: &[Bez3]) -> Self {
        Self {
            curves: Vec::from(slice),
        }
    }

    pub fn len(&self) -> usize {
        self.curves.len()
    }

    pub fn get(&self, u: f32) -> Option<Vec2> {
        self.get_split(u.floor() as usize, u - u.floor())
    }

    pub fn get_split(&self, i: usize, t: f32) -> Option<Vec2> {
        if i == self.len() {
            return Some(self.curves.last()?.last())
        }

        if i > self.len() {
            return None;
        }
    
        self.curves.get(i)?.get(t)
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
