use crate::{Bullet, Ship, Enemy};
use bevy::{
    prelude::*,
    sprite::collide_aabb::collide,
};

pub fn collide_player_enemy(
    mut cmd: Commands,

    ship_query: Query<&Transform, With<Ship>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    let ship_trans = ship_query.single();
    
    for (enemy_entity, enemy_trans) in &enemy_query {
        if let Some(_) = collide(
            ship_trans.translation,
            Vec2::new(50.0, 50.0),
            enemy_trans.translation,
            Vec2::new(5.0, 5.0)
        ) {
            cmd.entity(enemy_entity).despawn();
        };
    };
}

pub fn collide_bullet_enemy(
    mut cmd: Commands,

    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (enemy_entity, enemy_trans) in &enemy_query {
        for (bullet_entity, bullet_trans) in &bullet_query {
            if let Some(_) = collide(
                enemy_trans.translation,
                Vec2::new(5.0, 5.0),
                bullet_trans.translation,
                Vec2::new(5.0, 5.0),
            ) {
                cmd.entity(enemy_entity).despawn();
                cmd.entity(bullet_entity).despawn();
            };
        };
    };
}
