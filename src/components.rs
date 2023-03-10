use bevy::prelude::*;

#[derive(Resource)]
pub struct State {
    points: u32,
    health: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            points: 0,
            health: 3,
        }
    }
}

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