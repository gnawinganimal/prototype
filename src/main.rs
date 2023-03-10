
use std::{time::Duration, ops::Mul};

use std::cmp::max;
use bevy::{
    prelude::*,
    sprite::{
        MaterialMesh2dBundle,
        collide_aabb::{collide, Collision},
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .add_system(shoot)
        .add_system(move_ship)
        .add_system(move_bullet)
        .add_system(spawn_enemies)
        .add_system(check_for_collisions)
        .add_system(check_for_bullet_collisions)
        .add_system(despawn)
        .run();
}

const SHIP_SPEED: f32 = 250.0;
const BULLET_SPEED: f32 = 500.0;

#[derive(Resource)]
struct State {
    points: u32,
    health: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            points: 0,
            health: 3,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);

#[derive(Component)]
struct Ship;

#[derive(Component)]
struct Bullet;

#[derive(Resource)]
struct EnemyConfig {
    pub timer: Timer,
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Despawn(Timer);

// #[derive(Component)]
// struct Trajectory {
//     inner: Vec<Vec3>,
// }

// impl Trajectory {
//     pub fn new(slice: &[Vec3]) -> Self {
//         Self {
//             inner: Vec::from(slice),
//         }
//     }

//     pub fn get(&self, u: f32) -> Vec3 {
//         let mut p = Vec3::new(0.0, 0.0, 0.0);

//         let n = u.floor() as usize;
//         let t = u - u.floor();

//         for i in 0..n {
//             p += self.inner[i] * if i < n - 1 { 1.0 } else { t };
//         }

//         return p;
//     }
// }

// #[derive(Component)]
// struct EnemieGroup {
//     pub enemies: usize,

//     pub trajectory: Trajectory,
//     pub timer: Timer,
// }

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
    cmd.insert_resource(
        EnemyConfig {
            timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
        }
    );
    cmd.insert_resource(
        State::default(),
    )
}

fn move_ship(time: Res<Time>, input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Ship>>) {
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

    trans.translation += delta;
}

fn move_bullet(time: Res<Time>, mut query: Query<&mut Transform, With<Bullet>>) {
    for mut trans in &mut query {
        trans.translation.y += BULLET_SPEED * time.delta_seconds();
    }   
}

fn shoot(
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
            Despawn(Timer::new(Duration::from_secs(2), TimerMode::Once)),
            MaterialMesh2dBundle {
                mesh: mesh.add(shape::Circle::new(5.0).into()).into(),
                material: material.add(ColorMaterial::from(Color::ALICE_BLUE)),
                transform: trans.clone(),
                ..default()
            },
        ));
    }
}

fn spawn_enemies(
    mut cmd: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,

    mut config: ResMut<EnemyConfig>,
    time: Res<Time>,
) {
    config.timer.tick(time.delta());

    if config.timer.finished() {
        cmd.spawn((
            Enemy,
            MaterialMesh2dBundle {
                mesh: mesh.add(shape::Circle::new(5.0).into()).into(),
                material: material.add(ColorMaterial::from(Color::ALICE_BLUE)),
                transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
                ..default()
            },
        ));
    }
}

fn check_for_collisions(
    mut cmd: Commands,
    mut state: ResMut<State>,

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
            state.health -= 1;
            cmd.entity(enemy_entity).despawn();
        };
    };
}

fn check_for_bullet_collisions(
    mut cmd: Commands,
    mut state: ResMut<State>,

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
                state.points += 1000;
            };
        };
    };
}

fn despawn(mut cmd: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Despawn)>) {
    for (entity, mut despawn) in &mut query {
        despawn.0.tick(time.delta());
        if despawn.0.finished() {
            cmd.entity(entity).despawn();
        }
    }
}
