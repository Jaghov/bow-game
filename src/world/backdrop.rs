use std::time::Duration;

use avian3d::{
    math::PI,
    prelude::{Collider, CollisionLayers, RigidBody},
};
use bevy::{audio::Volume, color::palettes::tailwind::GREEN_400, math::ops::sin, prelude::*};
use bevy_tweening::{
    Animator, Delay, EaseMethod, Tween, TweenCompleted, lens::TransformPositionLens,
};

use crate::{
    asset_tracking::LoadResource,
    rand::{self, random_range},
    settings::Settings,
    third_party::avian3d::GameLayer,
    world::{BACKDROP_OFFSET, BLOCK_LEN, GAME_PLANE},
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_backdrop)
        .load_resource::<BackdropAssets>()
        .add_observer(play_backdrop_sfx)
        // .add_systems(OnEnter(LevelState::Playing), breathing_background)
    ;

    app.add_observer(pulse_out_backdrop_on_win);
}

#[derive(Event)]
pub struct RadialBackdropPulse;

const PERIOD: f32 = 0.3;

#[derive(Asset, Resource, Reflect, Clone)]
struct BackdropAssets {
    #[dependency]
    sfx: Handle<AudioSource>,
}

impl FromWorld for BackdropAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            sfx: asset_server.load("audio/sfx/block_shuffle.flac"),
        }
    }
}

#[derive(Component)]
struct ZState {
    spawn_depth: f32,
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
                    spawn_depth: z,
                    time_offset: std::hint::black_box(rand::random_range(
                        (-HALF_PERIOD..=HALF_PERIOD),
                    )),
                },
                CollisionLayers::new(
                    GameLayer::Backdrop,
                    [GameLayer::Backdrop, GameLayer::Arrow, GameLayer::Gibs],
                ),
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

fn sin_lerp(t: f32) -> f32 {
    sin(2. * PI * t)
}

fn pulse_out_backdrop_on_win(
    _: Trigger<RadialBackdropPulse>,
    mut commands: Commands,
    blocks: Query<(Entity, &mut Transform, &ZState)>,
) {
    for (block, mut transform, depth) in blocks {
        let delay = Duration::from_secs_f32((transform.translation.xy().length() / 120.) + 1.0);
        commands.entity(block).insert(Animator::new(
            // Bring blocks closer to intitial block plane
            Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(700),
                TransformPositionLens {
                    start: transform.translation,
                    end: {
                        transform.translation.z = depth.spawn_depth; // avg the z translation to soft reset backdrop
                        transform.translation
                    },
                },
            )
            // Pause to add suspense :)
            .then(Delay::new(delay).with_completed_event(0))
            // Pulse out from center
            .then(Tween::new(
                EaseMethod::CustomFunction(sin_lerp),
                Duration::from_secs_f32(PERIOD),
                TransformPositionLens {
                    start: transform.translation,
                    end: transform.translation - Vec3::new(0., 0., BLOCK_LEN), // End is more of an amplitude when using sin_lerp ease function
                },
            )),
        ));
    }
}

fn play_backdrop_sfx(
    trigger: Trigger<TweenCompleted>,
    mut commands: Commands,
    backdrop: Res<BackdropAssets>,
    settings: Res<Settings>,
) {
    if trigger.user_data != 0 {
        return;
    }
    commands.spawn((
        AudioPlayer(backdrop.sfx.clone()),
        PlaybackSettings {
            volume: Volume::Linear(0.05) * settings.sfx,
            speed: 1. / PERIOD,
            mode: bevy::audio::PlaybackMode::Remove,
            ..Default::default()
        },
    ));
}

/// Could be useful somewhere else maybe
#[allow(dead_code)]
fn breathing_background(
    mut commands: Commands,
    blocks: Query<(Entity, &mut Transform), With<ZState>>,
) {
    for (block, transform) in blocks {
        commands.entity(block).insert(Animator::new(
            Tween::new(
                EaseMethod::CustomFunction(sin_lerp),
                Duration::from_secs(random_range(2..5) * 5),
                TransformPositionLens {
                    start: transform.translation,
                    end: transform.translation - Vec3::new(0., 0., BLOCK_LEN / 4.),
                },
            )
            .with_repeat_count(bevy_tweening::RepeatCount::Infinite),
        ));
    }
}
