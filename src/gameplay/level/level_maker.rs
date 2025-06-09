use std::f32::consts::FRAC_PI_4;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

use crate::gameplay::level::wall::WallBuilder;
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
            horz!(4., -7., 8.),
            //vert!(-8., -5., 5.),
            horz!(2., -7., 6.),
            vert!(8., -5., 3.),
            vert!(6., -3., 1.),
            WallBuilder::block_rot(3., 6., 44.5, 20.5, FRAC_PI_4),
            WallBuilder::block_rot(3., 6., 44.5, -26.5, -FRAC_PI_4),
            horz!(-3., 4., 5.),
            horz!(-5., 4., 7.),
            horz!(-6., 0., 4.),
            horz!(-2., 0., 4.),
            vert!(-1., -6., -2.),
        ],
        vec![
            sphere!(Bouncy, 0., 18.),
            sphere!(Bouncy, 6., 18.),
            sphere!(Bouncy, 12., 18.),
            sphere!(Bouncy, 18., 18.),
            sphere!(Bouncy, 24., 18.),
            sphere!(Normal, 42., 0.),
            sphere!(Bouncy, 42., 6.),
            sphere!(Bouncy, 42., -6.),
            sphere!(Bouncy, 42., -12.),
            sphere!(Normal, 0., 0.),
            sphere!(Bouncy, 24., -24.),
            sphere!(Multiplier, 18., -24.),
            sphere!(Multiplier, 15., -24.),
            sphere!(Multiplier, 12., -24.),
            sphere!(Normal, 9., -24.),
            sphere!(Normal, 9., -21.),
            sphere!(Normal, 9., -27.),
            sphere!(Normal, 9., -30.),
            sphere!(Normal, 9., -18.),
            sphere!(Normal, 12., -27.),
            sphere!(Normal, 12., -21.),
            sphere!(Normal, 12., -18.),
            sphere!(Normal, 12., -30.),
            sphere!(Normal, 15., -27.),
            sphere!(Normal, 15., -21.),
            sphere!(Normal, 15., -18.),
            sphere!(Normal, 15., -30.),
            sphere!(Normal, 6., -24.),
            sphere!(Normal, 6., -21.),
            sphere!(Normal, 6., -27.),
            sphere!(Normal, 6., -30.),
            sphere!(Normal, 6., -18.),
            sphere!(Normal, 3., -24.),
            sphere!(Normal, 3., -21.),
            sphere!(Normal, 3., -27.),
            sphere!(Normal, 3., -30.),
            sphere!(Normal, 3., -18.),
            sphere!(Normal, 0., -24.),
            sphere!(Normal, 0., -21.),
            sphere!(Normal, 0., -27.),
            sphere!(Normal, 0., -30.),
            sphere!(Normal, 0., -18.),
        ],
    )
}
