use std::time::Duration;

use avian3d::prelude::RigidBody;
use bevy::prelude::*;

use crate::{
    gameplay::{
        GAMEPLAY_CAMERA_OFFSET, GameSet,
        bow::Quiver,
        level::{Level, LevelState, Levels, WallMaterial, new_level::timer::LevelSetupTimer},
        sphere::Sphere,
    },
    world::{GAME_PLANE, light::SetLightPosition},
};

const WALL_START_PLANE: f32 = GAMEPLAY_CAMERA_OFFSET + 20.;
const SPHERE_START_PLANE: f32 = GAME_PLANE - 20.;

mod timer;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(timer::plugin);

    app.add_systems(
        OnEnter(LevelState::NewLevel),
        (load_level, update_light_position).chain(),
    )
    .add_systems(
        Update,
        (update_wall_transform, update_sphere_transform)
            .in_set(GameSet::Update)
            .run_if(in_state(LevelState::NewLevel)),
    );
}

#[derive(Component)]
struct Walls;

fn load_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<WallMaterial>,
    mut levels: ResMut<Levels>,
    mut quiver: ResMut<Quiver>,
    level: Res<Level>,
) {
    let props = levels.get(level.0);

    let root = commands
        .spawn((
            Walls,
            Transform::from_xyz(0., 0., WALL_START_PLANE),
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

    //todo
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

fn update_light_position(
    mut commands: Commands,
    //time: Res<LevelSetupTimer>,
    //mut done: Local<bool>,
) {
    // if *done || !time.should_move_light() {
    //     return;
    // }

    commands.trigger(SetLightPosition::to_above().with_duration(Duration::from_millis(700)));

    //*done = true;
}
