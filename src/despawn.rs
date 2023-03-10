use bevy::{prelude::*, time::Timer};

#[derive(Component)]
pub struct DespawnTimer(pub Timer);

pub fn despawn_timer_system(mut cmd: Commands, time: Res<Time>, mut query: Query<(Entity, &mut DespawnTimer)>) {
    for (entity, mut despawn) in &mut query {
        despawn.0.tick(time.delta());
        if despawn.0.finished() {
            cmd.entity(entity).despawn();
        }
    }
}
