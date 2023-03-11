
use crate::{SHIP_SPEED, BULLET_SIZE, Bullet, Ship, despawn::DespawnTimer};
use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use std::time::Duration;

pub fn move_ship(time: Res<Time>, input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Ship>>) {
    let mut trans = query.single_mut();
    let mut delta = Vec3::from_array([0.0, 0.0, 0.0]);

    if input.pressed(KeyCode::W) {
        delta.y = SHIP_SPEED * time.delta_seconds();
    }

    if input.pressed(KeyCode::A) {
        delta.x = -SHIP_SPEED * time.delta_seconds();
    }

    if input.pressed(KeyCode::S) {
        delta.y = -SHIP_SPEED * time.delta_seconds();
    }

    if input.pressed(KeyCode::D) {
        delta.x = SHIP_SPEED * time.delta_seconds();
    }

    let temp = trans.translation + delta;

    if !(delta.x > 0. && temp.x > 100.) && !(delta.x < 0. && temp.x < -100.) {
        trans.translation.x = temp.x;
    }

    if !(delta.y > 0. && temp.y > 100.) && !(delta.y < 0. && temp.y < -100.) {
        trans.translation.y = temp.y;
    }
}

pub fn shoot(
    mut cmd: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,

    input: Res<Input<KeyCode>>,
    query: Query<&mut Transform, With<Ship>>
) {
    let trans = query.single();
    
    if input.just_pressed(KeyCode::Space) {
        cmd.spawn((
            Bullet,
            // DespawnTimer(Timer::new(Duration::from_secs(2), TimerMode::Once)),
            MaterialMesh2dBundle {
                mesh: mesh.add(shape::Circle::new(BULLET_SIZE).into()).into(),
                material: material.add(ColorMaterial::from(Color::ALICE_BLUE)),
                transform: trans.clone(),
                ..default()
            },
        ));
    }
}
