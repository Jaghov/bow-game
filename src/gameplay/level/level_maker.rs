use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

use crate::gameplay::level::{
    Level, LevelProps, LevelState, WallMaterial, WallMesh, Walls, sphere::SphereType,
};
use crate::settings::Settings;
use crate::third_party::avian3d::GameLayer;
use crate::world::GAME_PLANE;

// this will hot reload level 0 forever
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, change_level.run_if(in_state(LevelState::Playing)))
        .add_systems(OnEnter(LevelState::Playing), set_dev_level)
        .add_systems(
            Update,
            (set_dev_level_update, infinite_mulligans).run_if(in_state(LevelState::Playing)),
        );
}

fn inner(
    mut commands: Commands,
    meshes: &mut Assets<Mesh>,
    walls: Entity,
    material: &WallMaterial,
    spheres: Query<Entity, With<SphereType>>,
) {
    let props = edit_level();

    commands.entity(walls).despawn_related::<Children>();

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
            ChildOf(walls),
        ));
    }

    for sphere in spheres {
        commands.entity(sphere).despawn();
    }

    for sphere in props.spheres.iter() {
        commands.spawn((
            sphere.sphere_type,
            Transform::from_xyz(sphere.location.x, sphere.location.y, GAME_PLANE),
        ));
    }
}

#[hot]
fn set_dev_level(
    commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<WallMaterial>,
    walls: Single<Entity, With<Walls>>,
    spheres: Query<Entity, With<SphereType>>,
) {
    inner(commands, &mut meshes, *walls, &material, spheres);
}

fn infinite_mulligans(
    input: Res<ButtonInput<KeyCode>>,
    settings: Res<Settings>,
    mut level_state: ResMut<NextState<LevelState>>,
) {
    if input.just_pressed(settings.restart) {
        level_state.set(LevelState::NextLevel);
    }
}

#[hot]
fn set_dev_level_update(
    commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<WallMaterial>,
    walls: Single<Entity, With<Walls>>,
    spheres: Query<Entity, With<SphereType>>,
) {
    if !should_be_update_reloading() {
        return;
    }
    warn!("HOT PATCH UPDATING IS ENABLED");
    inner(commands, &mut meshes, *walls, &material, spheres);
}
#[cfg(feature = "dev")]
fn change_level(
    keys: Res<ButtonInput<KeyCode>>,
    mut level: ResMut<Level>,
    mut state: ResMut<NextState<LevelState>>,
) {
    let mut cs = false;
    if keys.just_pressed(KeyCode::Semicolon) {
        level.0 += 1;
        cs = true;
    }
    if keys.just_pressed(KeyCode::Slash) {
        level.0 = level.0.saturating_sub(level.0);
        cs = true;
    }
    if cs {
        state.set(LevelState::NewLevel);
    }
}

fn should_be_update_reloading() -> bool {
    false
}
/*
- simple normals
- simple multiplier
- advanced multiplier
- multiplier + exploder
- timefreeze TODO
- Advanced 1 (done)
*/

fn edit_level() -> LevelProps {
    //32 on bounds
    // making a spiral
    LevelProps::new(
        2,
        vec![
            vert!(-8., -5., 6.),
            horz!(-6., -8., 8.),
            vert!(8., -5., 5.),
            horz!(6., -4., 8.),
            vert!(-4., -2., 5.),
            horz!(-2., -3., 4.),
            vert!(4., -1., 1.),
            horz!(2., 0., 4.),
        ],
        vec![
            sphere!(Normal, -36., 24.),
            sphere!(TimeFreeze, -36., -24.),
            sphere!(TimeFreeze, 36., -24.),
            sphere!(TimeFreeze, 36., 24.),
            sphere!(TimeFreeze, -12., 24.),
            sphere!(TimeFreeze, -12., 0.),
            sphere!(Multiplier, 10., 0.),
            sphere!(Normal, 18., 0.),
            sphere!(Normal, 18., 6.),
            sphere!(Normal, 18., -6.),
        ],
    )
}
