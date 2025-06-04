use avian3d::prelude::{Collider, RigidBody};
use bevy::{color::palettes::tailwind::GREEN_400, prelude::*};

use crate::{
    rand,
    world::{BACKDROP_OFFSET, BLOCK_LEN, GAME_PLANE},
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_backdrop);
    //.add_systems(Update, update_backdrop_z.in_set(GameSet::Update));
}

const PERIOD: f32 = 0.3;

#[derive(Component)]
struct ZState {
    time_offset: f32,
}

fn spawn_backdrop(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        base_color: GREEN_400.into(),
        ..default()
    });

    let mesh = meshes.add(Cuboid::from_length(BLOCK_LEN));

    for i in (-10..=10) {
        for j in (-10..=10) {
            let x = i as f32 * BLOCK_LEN;
            let y = j as f32 * BLOCK_LEN;
            let depth = BLOCK_LEN / 2.;
            let z = GAME_PLANE
                - BACKDROP_OFFSET
                - rand::random_range((0_f32..=depth))
                - (BLOCK_LEN * 0.5);

            let spawn_here = rand::random_bool(0.9);
            if !spawn_here {
                continue;
            }
            const HALF_PERIOD: f32 = PERIOD * 0.5;
            commands.spawn((
                MeshMaterial3d(material.clone()),
                ZState {
                    time_offset: rand::random_range((-HALF_PERIOD..=HALF_PERIOD)),
                },
                Mesh3d(mesh.clone()),
                RigidBody::Kinematic,
                Collider::cuboid(BLOCK_LEN, BLOCK_LEN, BLOCK_LEN),
                Transform::from_xyz(x, y, z),
            ));
        }
    }
}

#[allow(dead_code)]
fn update_backdrop_z(mut blocks: Query<(&mut Transform, &mut ZState)>, time: Res<Time>) {
    const TRVL: f32 = BACKDROP_OFFSET * 0.5;

    for (mut trns, zstate) in &mut blocks {
        let progress_time = (time.elapsed_secs() + zstate.time_offset) % PERIOD;
        if progress_time > PERIOD * 0.5 {
            trns.translation.z += TRVL * time.delta_secs()
            //forward
        } else {
            trns.translation.z -= TRVL * time.delta_secs()
            //backward
        }
    }
}
