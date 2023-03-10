use bevy::prelude::*;

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
pub struct Follow {
    pub path: Bezier<Vec2>,
    pub timer: Timer,
}
