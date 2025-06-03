use bevy::prelude::*;

use crate::{
    Screen,
    world::{GAME_PLANE, backdrop::BLOCK_LEN},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_walls)
        .add_systems(Update, set_wall_locs.run_if(in_state(Screen::Gameplay)));
}

#[derive(Component)]
struct Top;
#[derive(Component)]
struct Right;
#[derive(Component)]
struct Bottom;

fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //top wall
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });
    let top_mesh = meshes.add(Cuboid::new(BLOCK_LEN * 19., BLOCK_LEN, BLOCK_LEN));

    commands.spawn((
        Mesh3d(top_mesh.clone()),
        Top,
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(-BLOCK_LEN * 5., 0., GAME_PLANE),
    ));

    let right_mesh = meshes.add(Cuboid::new(BLOCK_LEN, BLOCK_LEN * 7., BLOCK_LEN));

    commands.spawn((
        Mesh3d(right_mesh),
        Right,
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(BLOCK_LEN * 9., 0., GAME_PLANE),
    ));

    commands.spawn((
        Mesh3d(top_mesh),
        Bottom,
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(-BLOCK_LEN * 5., 0., GAME_PLANE),
    ));

    //let
}

#[cfg_attr(feature = "hot", bevy_simple_subsecond_system::prelude::hot)]
fn set_wall_locs(
    mut top: Single<&mut Transform, (With<Top>, Without<Right>, Without<Bottom>)>,
    mut right: Single<&mut Transform, (With<Right>, Without<Bottom>, Without<Top>)>,
    mut bottom: Single<&mut Transform, (With<Bottom>, Without<Right>, Without<Top>)>,
) {
    **top = Transform::from_xyz(0., BLOCK_LEN * 4., GAME_PLANE);
    **right = Transform::from_xyz(BLOCK_LEN * 9., 0., GAME_PLANE);
    **bottom = Transform::from_xyz(0., -BLOCK_LEN * 4., GAME_PLANE);
    //todo
}
