use std::time::Duration;

pub use crate::{
    components::{Ship, Bullet, Enemy},
    controls::{move_ship, shoot},
    movement::move_bullet,
    collision::{collide_player_enemy, collide_bullet_enemy},
    despawn::despawn_timer_system,
};
use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use components::Follow;
use spline::Path;

pub mod despawn;
pub mod controls;
pub mod collision;
pub mod components;
pub mod movement;
pub mod spline;

pub const SHIP_SPEED: f32 = 250.0;
pub const BULLET_SPEED: f32 = 500.0;

fn follow(mut cmd: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Transform, &mut Follow)>) {
    for (entity, mut trans, mut follow) in &mut query {
        // increment time
        follow.timer.tick(time.delta());

        // update position
        if let Some(vec2) = follow.path.get(follow.timer.elapsed().as_secs_f32()) {
            trans.translation.x = vec2.x as f32;
            trans.translation.y = vec2.y as f32;
        }

        // despawn entities which have finished their paths
        if follow.timer.finished() {
            cmd.entity(entity).despawn();
        }
    }
}

fn startup(
    mut cmd: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    cmd.spawn(Camera2dBundle::default());
    cmd.spawn((
        Ship,
        MaterialMesh2dBundle {
            mesh: mesh.add(shape::Circle::new(20.0).into()).into(),
            material: material.add(ColorMaterial::from(Color::ALICE_BLUE)),
            transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
            ..default()
        },
    ));
    cmd.spawn((
        Enemy,
        Follow {
            timer: Timer::from_seconds(3.0, TimerMode::Once),
            path: Path::new(&[Vec2::new(0.0, 0.0), Vec2::new(100.0, 0.0), Vec2::new(200.0, 0.0), Vec2::new(100.0, 0.0)]),
        },
        MaterialMesh2dBundle {
            mesh: mesh.add(shape::Circle::new(20.0).into()).into(),
            material: material.add(ColorMaterial::from(Color::ALICE_BLUE)),
            transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
            ..default()
        },
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .add_system(move_ship)
        .add_system(shoot)
        .add_system(move_bullet)
        .add_system(collide_player_enemy)
        .add_system(collide_bullet_enemy)
        .add_system(despawn_timer_system)
        .add_system(follow)
        .run();
}
