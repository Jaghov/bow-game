use std::time::Duration;

use avian3d::prelude::{Collider, RigidBody};
use bevy::{color::palettes::tailwind::GREEN_400, ecs::system::SystemId, prelude::*};
use bevy_tweening::{Animator, Delay, Sequence, Tween, lens::TransformPositionLens};

use crate::{
    rand,
    world::{BACKDROP_OFFSET, BLOCK_LEN, GAME_PLANE},
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_backdrop);

    let backdrop_win = app.register_system(pulse_out_backdrop_on_win);

    app.insert_resource(RadialBackdropPulse(backdrop_win));
}

#[derive(Resource)]
pub struct RadialBackdropPulse(pub SystemId);

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

fn pulse_out_backdrop_on_win(
    mut commands: Commands,
    blocks: Query<(Entity, &mut Transform), With<ZState>>,
) {
    for (block, transform) in blocks {
        let delay = Duration::from_secs_f32((transform.translation.xy().length() / 120.) + 0.2);
        info!("Delay is {:#?}", delay);
        commands.entity(block).insert(Animator::new(
            Sequence::from_single(Delay::new(delay)).then(
                Tween::new(
                    EaseFunction::SineIn,
                    Duration::from_millis(100),
                    TransformPositionLens {
                        start: transform.translation,
                        end: transform.translation - Vec3::new(0., 0., BLOCK_LEN),
                    },
                )
                .then(Tween::new(
                    EaseFunction::SineOut,
                    Duration::from_millis(100),
                    TransformPositionLens {
                        start: transform.translation - Vec3::new(0., 0., BLOCK_LEN),
                        end: transform.translation,
                    },
                )),
            ),
        ));
    }
}
