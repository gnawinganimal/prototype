use bevy::prelude::*;
use crate::Bezier;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec3);

#[derive(Component)]
pub struct Ship;

#[derive(Component)]
pub struct Bullet;

#[derive(Resource)]
pub struct EnemyConfig {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Follower {
    pub path: Bezier,
    pub u: f32,
}
