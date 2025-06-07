use std::time::Duration;

use avian3d::prelude::{CollisionLayers, PhysicsLayer, RigidBody};
use bevy::prelude::*;
use bevy_tweening::{Animator, Tween, lens::TransformPositionLens};

use crate::{
    gameplay::{
        GameSet,
        bow::Quiver,
        level::{
            Level, LevelState, Levels, SPHERE_START_PLANE, WALL_START_PLANE, WallMaterial,
            WallMesh, Walls, sphere::SphereType, timer::LevelSetupTimer,
        },
        sphere::Sphere,
    },
    third_party::avian3d::GameLayer,
    world::{GAME_PLANE, backdrop::RadialBackdropPulse, light::SetLightPosition},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
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
        observe_level_completion.run_if(in_state(LevelState::Playing)),
    )
    .add_systems(
        Update,
        hot_reloading_walls.run_if(in_state(LevelState::Playing)),
    );
}
fn init_timer(mut commands: Commands) {
    commands.init_resource::<LevelSetupTimer>();
}
#[derive(Default)]
struct LevelCompletion {
    timer: Option<Timer>,
}
fn observe_level_completion(
    mut commands: Commands,
    balls: Query<&SphereType, With<Sphere>>,
    mut level: ResMut<Level>,
    mut next_state: ResMut<NextState<LevelState>>,
    backdrop_pulse: Res<RadialBackdropPulse>,
    mut level_completion: Local<LevelCompletion>,
    time: Res<Time>,
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

    // Check if only bouncy, absorber, or gravity balls remain (these don't count toward completion)
    let remaining_balls_count = balls
        .iter()
        .filter(|sphere_type| {
            !matches!(
                sphere_type,
                SphereType::Bouncy | SphereType::Absorber | SphereType::Gravity
            )
        })
        .count();

    if remaining_balls_count == 0 {
        commands.run_system(backdrop_pulse.0);
        level_completion.timer = Some(Timer::new(Duration::from_millis(2000), TimerMode::Once));
    }
}

#[cfg_attr(feature = "hot", bevy_simple_subsecond_system::prelude::hot)]
fn hot_reloading_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<WallMaterial>,
    mut levels: ResMut<Levels>,
    level: Res<Level>,
    walls: Single<Entity, With<Walls>>,
) {
    let props = levels.get(level.0);
}

fn load_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<WallMaterial>,
    mut levels: ResMut<Levels>,
    mut quiver: ResMut<Quiver>,
    level: Res<Level>,
    timer: Res<LevelSetupTimer>,
) {
    let props = levels.get(level.0);

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

    quiver.set_arrow_count(props.arrow_count);

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
    commands.trigger(SetLightPosition::to_above().with_duration(Duration::from_millis(700)));
}

fn update_level_state(timer: Res<LevelSetupTimer>, mut level_state: ResMut<NextState<LevelState>>) {
    if timer.finished() {
        level_state.set(LevelState::Playing);
    }
}
