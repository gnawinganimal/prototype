use bevy::{prelude::*, time::Timer};
use crate::Bullet;

#[derive(Component)]
pub struct DespawnTimer(pub Timer);

pub fn despawn_bullets(mut cmd: Commands, query: Query<(Entity, &Transform), With<Bullet>>) {
    for (entity, trans) in &query {
        if trans.translation.y > 100. + 50. {
            cmd.entity(entity).despawn();
        }
    }
}

pub fn despawn_timer_system(mut cmd: Commands, time: Res<Time>, mut query: Query<(Entity, &mut DespawnTimer)>) {
    for (entity, mut despawn) in &mut query {
        despawn.0.tick(time.delta());
        if despawn.0.finished() {
            cmd.entity(entity).despawn();
        }
    }
}
