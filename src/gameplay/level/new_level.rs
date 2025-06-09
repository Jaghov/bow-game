use std::{path::Path, time::Duration};

use avian3d::prelude::{CollisionLayers, PhysicsLayer, RigidBody};
use bevy::prelude::*;
use bevy_tweening::{Animator, Tween, lens::TransformPositionLens};

use crate::{
    asset_tracking::LoadResource,
    gameplay::{
        GameSet,
        gameover::GameOverState,
        level::{
            Level, LevelState, Levels, SPHERE_START_PLANE, WALL_START_PLANE, WallMaterial,
            WallMesh, Walls, timer::LevelSetupTimer,
        },
        sphere::{MarkedForDeletion, MustMark, Sphere},
    },
    settings::Settings,
    third_party::avian3d::GameLayer,
    world::{GAME_PLANE, backdrop::RadialBackdropPulse, light::SetLightPosition},
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>()
        .add_systems(
            OnEnter(LevelState::NewLevel),
            (init_timer, load_level, set_light_position).chain(),
        )
        .add_systems(
            Update,
            (
                // update_wall_transform,
                update_sphere_transform
            )
                .in_set(GameSet::Update)
                .run_if(in_state(LevelState::NewLevel)),
        )
        .add_systems(
            PostUpdate,
            update_level_state.run_if(in_state(LevelState::NewLevel)),
        )
        .add_systems(
            Update,
            observe_level_completion
                .run_if(in_state(LevelState::Playing).and(in_state(GameOverState::None))),
        );
}
fn init_timer(mut commands: Commands) {
    commands.init_resource::<LevelSetupTimer>();
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct LevelAssets {
    #[dependency]
    level_complete_sfx: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            level_complete_sfx: assets.load(Path::new("audio/sfx/level_complete.flac")),
        }
    }
}

#[derive(Default)]
struct LevelCompletion {
    timer: Option<Timer>,
}
fn observe_level_completion(
    mut commands: Commands,
    sensor_balls: Query<(), (With<Sphere>, Without<MustMark>)>,
    markable_balls: Query<(), (With<MustMark>, Without<MarkedForDeletion>)>,
    mut level: ResMut<Level>,
    mut next_state: ResMut<NextState<LevelState>>,
    mut level_completion: Local<LevelCompletion>,
    time: Res<Time>,
    sfx: Res<LevelAssets>,
    settings: Res<Settings>,
) {
    let mut reset_timer = false;
    if let Some(timer) = &mut level_completion.timer {
        timer.tick(time.delta());
        if !timer.just_finished() {
            return;
        }

        level.0 += 1;
        next_state.set(LevelState::NextLevel);
        reset_timer = true;
    }
    if reset_timer {
        level_completion.timer = None;
        return;
    }

    let remaining_balls_count = sensor_balls.iter().count() + markable_balls.iter().count();

    if remaining_balls_count == 0 {
        #[cfg(all(not(feature = "web"), not(feature = "webgpu")))]
        commands.trigger(RadialBackdropPulse);
        commands.spawn((
            AudioPlayer::new(sfx.level_complete_sfx.clone()),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: settings.sfx,
                // speed: random_range(0.9..1.1),
                ..Default::default()
            },
        ));
        level_completion.timer = Some(Timer::new(Duration::from_millis(2000), TimerMode::Once));
    }
}

fn load_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<WallMaterial>,
    mut levels: ResMut<Levels>,
    level: Res<Level>,
    timer: Res<LevelSetupTimer>,
) {
    let Some(props) = levels.get(level.0) else {
        // this should probably panic, but yknow
        return;
    };

    let tween = Tween::new(
        EaseFunction::QuadraticOut,
        timer.wall_duration(),
        TransformPositionLens {
            start: Vec3 {
                x: 0.,
                y: 0.,
                z: WALL_START_PLANE,
            },
            end: Vec3::ZERO,
        },
    );

    let root = commands
        .spawn((
            Name::new("Walls"),
            Walls,
            CollisionLayers::new(
                GameLayer::Walls,
                [GameLayer::Default, GameLayer::ArrowSensor],
            ),
            Transform::from_xyz(0., 0., WALL_START_PLANE),
            Animator::new(tween),
            InheritedVisibility::VISIBLE,
            RigidBody::Static,
        ))
        .id();

    for wall in props.walls.iter() {
        let collider = wall.collider.clone();
        let mesh = match wall.mesh {
            WallMesh::Cuboid(cuboid) => meshes.add(cuboid),
            WallMesh::Cylinder(cylinder) => meshes.add(cylinder),
        };
        let material = material.0.clone();
        commands.spawn((
            Mesh3d(mesh),
            collider,
            MeshMaterial3d(material),
            CollisionLayers::new(GameLayer::Walls, GameLayer::all_bits()),
            wall.transform,
            ChildOf(root),
        ));
    }
    for sphere in props.spheres.iter() {
        commands.spawn((
            sphere.sphere_type,
            Transform::from_xyz(sphere.location.x, sphere.location.y, SPHERE_START_PLANE),
        ));
    }
}

fn update_sphere_transform(
    time: Res<LevelSetupTimer>,
    mut spheres: Query<&mut Transform, (With<Sphere>, Without<Walls>)>,
) {
    let progress = time.sphere_progress();
    let eased_progress = progress * progress * (3.0 - 2.0 * progress);

    let sphere_z = SPHERE_START_PLANE.lerp(GAME_PLANE, eased_progress);
    for mut sphere in &mut spheres {
        sphere.translation.z = sphere_z;
    }
}

fn set_light_position(mut commands: Commands) {
    commands.trigger(SetLightPosition::to_gameplay().with_duration(Duration::from_millis(700)));
}

fn update_level_state(timer: Res<LevelSetupTimer>, mut level_state: ResMut<NextState<LevelState>>) {
    if timer.finished() {
        level_state.set(LevelState::Playing);
    }
}
