
pub use crate::{
    components::{Ship, Bullet, Enemy},
    controls::{move_ship, shoot},
    movement::move_bullet,
    collision::{collide_player_enemy, collide_bullet_enemy},
    despawn::despawn_timer_system,
    spline::{Path, Bez3},
};
use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use components::Follow;

pub mod despawn;
pub mod controls;
pub mod collision;
pub mod components;
pub mod movement;
pub mod spline;

pub const SHIP_SPEED: f32 = 250.0;
pub const BULLET_SPEED: f32 = 500.0;

pub const PLAYER_SIZE: f32 = 15.;
pub const BULLET_SIZE: f32 = 5.;
pub const ENEMY_SIZE: f32 = 15.;

fn follow(mut cmd: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Transform, &mut Follow)>) {
    for (entity, mut trans, mut follow) in &mut query {
        // increment time
        follow.timer.tick(time.delta());

        let p = follow.path.to_curve().position(follow.timer.elapsed().as_secs_f32());
        trans.translation.x = p.x;
        trans.translation.y = p.y;

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
            mesh: mesh.add(shape::Circle::new(PLAYER_SIZE).into()).into(),
            material: material.add(ColorMaterial::from(Color::ALICE_BLUE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
    ));
    cmd.spawn((
        Enemy,
        Follow {
            timer: Timer::from_seconds(2.0, TimerMode::Once),
            path: Bezier::new([
                [
                    Vec2::new(-300.0, -300.0),
                    Vec2::new(-300.0, -200.0),
                    Vec2::new(-200.0, -100.0),
                    Vec2::new(-100.0, -100.0),
                ],
                [
                    Vec2::new(-100.0, -100.0),
                    Vec2::new(0.0, -100.0),
                    Vec2::new(100.0, 0.0),
                    Vec2::new(100.0, 100.0),
                ],
            ]), 
        },
        MaterialMesh2dBundle {
            mesh: mesh.add(shape::Circle::new(ENEMY_SIZE).into()).into(),
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
