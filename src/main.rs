
pub use crate::{
    components::{Ship, Bullet, Enemy},
    controls::{move_ship, shoot},
    movement::move_bullet,
    collision::{collide_player_enemy, collide_bullet_enemy},
    despawn::despawn_timer_system,
    spline::{Bezier},
};
use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use components::Follower;

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
pub const ENEMY_SPEED: f32 = 60.0;

fn follow(mut cmd: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Transform, &mut Follower)>) {
    for (entity, mut trans, mut follower) in &mut query {
        let t = time.delta_seconds();
        let s = ENEMY_SPEED * t;

        if follower.u > follower.path.len() {
            cmd.entity(entity).despawn();
            continue;
        }

        let [p0, p1, p2, p3] = follower.path.get_curve(follower.u).expect("Curve does not exist");

        let v0 = -3. * p0 + 9. * p1 - 9. * p2 + 3. * p3;
        let v1 = 6. * p0 - 12. * p1 + 6. * p2;
        let v2 = -3. * p0 + 3. * p1;

        follower.u += s / (t.powi(2) * v0 + t * v1 + v2).length();
        let p = follower.path.get(follower.u).expect("AHHH!");
        trans.translation.x = p.x;
        trans.translation.y = p.y;
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
        Follower {
            u: 0.,
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
