use avian3d::prelude::*;
use bevy::prelude::*;

use crate::gameplay::bow::Quiver;
use crate::gameplay::level::{
    Level, LevelProps, LevelState, WallMaterial, Walls, sphere::SphereType,
};
use crate::third_party::avian3d::GameLayer;
use crate::world::GAME_PLANE;

// this will hot reload level 0 forever
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, change_level.run_if(in_state(LevelState::Playing)))
        .add_systems(OnEnter(LevelState::Playing), set_dev_level);
}

#[cfg_attr(feature = "hot", bevy_simple_subsecond_system::prelude::hot)]
fn set_dev_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<WallMaterial>,
    mut quiver: ResMut<Quiver>,
    walls: Single<Entity, With<Walls>>,
    spheres: Query<Entity, With<SphereType>>,
) {
    if !should_be_reloading() {
        return;
    }
    let props = edit_level();

    commands.entity(*walls).despawn_related::<Children>();

    for wall in props.walls.iter() {
        let collider = wall.collider.clone();
        let mesh = meshes.add(wall.mesh);
        let material = material.0.clone();
        commands.spawn((
            Mesh3d(mesh),
            collider,
            MeshMaterial3d(material),
            CollisionLayers::new(GameLayer::Walls, GameLayer::all_bits()),
            wall.transform,
            ChildOf(*walls),
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
fn should_be_reloading() -> bool {
    true
}

fn edit_level() -> LevelProps {
    LevelProps::new(
        None,
        vec![
            vert!(6., -4., 4.),
            horz!(5., -6., 6.),
            vert!(-6., -4., 4.),
            horz!(-5., -6., 6.),
            vert!(0., 1., 4.),
            vert!(0., -4., -1.),
        ],
        vec![
            sphere!(Normal, -17., 13.),
            sphere!(Multiplier, -7., 21.),
            sphere!(Multiplier, 7., 21.),
            sphere!(Normal, 15., 21.),
            sphere!(Normal, 15., 13.),
            // sphere!(Normal, 10., 5.),
            // sphere!(Normal, 10., -5.),
        ],
    )
}
