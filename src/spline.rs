use bevy::prelude::*;

// currently path is simply collection of straight lines

// also note - paths must contain at least two points to be valid

pub struct Path {
    points: Vec<Vec2>,
}

impl Path {
    pub fn new(slice: &[Vec2]) -> Self {
        Self {
            points: Vec::from(slice),
        }
    }

    pub fn len(&self) -> usize {
        self.points.len() - 1
    }

    pub fn get(&self, u: f32) -> Option<Vec2> {
        self.get_split(u.floor() as usize, u - u.floor())
    }

    pub fn get_split(&self, i: usize, t: f32) -> Option<Vec2> {
        if i > self.len() - 1 {
            return None;
        }

        let a = self.points[i];
        let d = self.points[i + 1] - a;
        
        Some(Vec2::new(d.x * t, d.y * t) + a)
    }
}
