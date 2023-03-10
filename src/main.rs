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

pub mod despawn;
pub mod controls;
pub mod collision;
pub mod components;
pub mod movement;

pub const SHIP_SPEED: f32 = 250.0;
pub const BULLET_SPEED: f32 = 500.0;

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
        .run();
}
