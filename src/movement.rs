use crate::{BULLET_SPEED, Bullet};
use bevy::prelude::*;

pub fn move_bullet(time: Res<Time>, mut query: Query<&mut Transform, With<Bullet>>) {
    for mut trans in &mut query {
        trans.translation.y += BULLET_SPEED * time.delta_seconds();
    }   
}
