use avian3d::prelude::*;
use bevy::prelude::*;

use crate::gameplay::bow::Quiver;
use crate::gameplay::level::WallMesh;
use crate::gameplay::level::wall::WallBuilder;
use crate::gameplay::level::{
    Level, LevelProps, LevelState, WallMaterial, Walls, sphere::SphereType,
};
use crate::third_party::avian3d::GameLayer;
use crate::world::GAME_PLANE;

// this will hot reload level 0 forever
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, change_level.run_if(in_state(LevelState::Playing)))
        .add_systems(OnEnter(LevelState::Playing), set_dev_level)
        .add_systems(
            Update,
            set_dev_level_update.run_if(in_state(LevelState::Playing)),
        );
}

fn inner(
    mut commands: Commands,
    meshes: &mut Assets<Mesh>,
    walls: Entity,
    material: &WallMaterial,
    quiver: &mut Quiver,
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

    quiver.set_arrow_count(props.arrow_count);
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

#[cfg_attr(feature = "hot", bevy_simple_subsecond_system::prelude::hot)]
fn set_dev_level(
    commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<WallMaterial>,
    mut quiver: ResMut<Quiver>,
    walls: Single<Entity, With<Walls>>,
    spheres: Query<Entity, With<SphereType>>,
) {
    inner(
        commands,
        &mut meshes,
        *walls,
        &material,
        &mut quiver,
        spheres,
    );
}

#[cfg_attr(feature = "hot", bevy_simple_subsecond_system::prelude::hot)]
fn set_dev_level_update(
    commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<WallMaterial>,
    mut quiver: ResMut<Quiver>,
    walls: Single<Entity, With<Walls>>,
    spheres: Query<Entity, With<SphereType>>,
) {
    if !should_be_update_reloading() {
        return;
    }
    inner(
        commands,
        &mut meshes,
        *walls,
        &material,
        &mut quiver,
        spheres,
    );
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
    true
}

fn edit_level() -> LevelProps {
    LevelProps::new(
        None,
        vec![
            //right
            vert!(7., -4., 6.),
            //horz left
            horz!(5., -7., 1.),
            //left
            vert!(-7., -4., 4.),
            //bottom
            horz!(-5., -7., 7.),
            //divider top
            vert!(2., 4., 6.),
            //horz top right
            horz!(6., 3., 6.),
            //div bot
            vert!(2., -4., 2.),
            WallBuilder::pole(0.5, 26., -10.),
        ],
        vec![
            //left side
            sphere!(Normal, -18., -2.),
            //right side
            sphere!(Exploder, 21., 20.),
            sphere!(Normal, 17., 14.),
            sphere!(Normal, 17., 24.),
            sphere!(TimeFreeze, 26., 21.),
            //bowling
            sphere!(Multiplier, 26., -12.),
        ],
    )
}
