use std::time::Duration;

use avian3d::prelude::{CollisionLayers, RigidBody};
use bevy::prelude::*;
use bevy_tweening::{Animator, Tween, lens::TransformPositionLens};

use crate::{
    gameplay::{
        GameSet,
        bow::Quiver,
        level::{
            Level, LevelState, Levels, SPHERE_START_PLANE, WALL_START_PLANE, WallMaterial, Walls,
            timer::LevelSetupTimer,
        },
        sphere::Sphere,
    },
    third_party::avian3d::GameLayer,
    world::{GAME_PLANE, light::SetLightPosition},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(LevelState::NewLevel),
        (init_timer, (load_level, set_light_position).chain()),
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
    );
}
fn init_timer(mut commands: Commands) {
    commands.init_resource::<LevelSetupTimer>();
}

fn observe_level_completion(
    balls: Query<(), With<Sphere>>,
    mut level: ResMut<Level>,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    if balls.is_empty() {
        level.0 += 1;
        next_state.set(LevelState::NextLevel);
    }
}

fn load_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<WallMaterial>,
    mut levels: ResMut<Levels>,
    mut quiver: ResMut<Quiver>,
    level: Res<Level>,
) {
    let props = levels.get(level.0);

    let tween = Tween::new(
        EaseFunction::BounceOut,
        Duration::from_secs(2),
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
            Walls,
            CollisionLayers::new(GameLayer::Default, GameLayer::Default),
            Transform::from_xyz(0., 0., WALL_START_PLANE),
            Animator::new(tween),
            InheritedVisibility::VISIBLE,
            RigidBody::Static,
        ))
        .id();

    quiver.set_arrow_count(props.arrow_count);

    for wall in props.walls.iter() {
        let collider = wall.collider.clone();
        let mesh = meshes.add(wall.mesh);
        let material = material.0.clone();
        commands.spawn((
            Mesh3d(mesh),
            collider,
            MeshMaterial3d(material),
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

fn update_wall_transform(
    time: Res<LevelSetupTimer>,
    mut walls: Query<&mut Transform, (With<Walls>, Without<Sphere>)>,
) {
    let mut walls = walls
        .single_mut()
        .expect("No wall for level loading. This is unrecoverable!");

    let progress = time.wall_progress();
    let eased_progress = progress * progress * (3.0 - 2.0 * progress);

    let wall_z = WALL_START_PLANE.lerp(GAME_PLANE, eased_progress);

    walls.translation.z = wall_z;
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
