use bevy::{
    color::palettes::tailwind::{GREEN_400, GREEN_500},
    prelude::*,
};

use crate::gameplay::GAME_PLANE;

use super::GameLoadState;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameLoadState::Loaded), spawn_backdrop);
}

fn spawn_backdrop(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const BL: f32 = 6.;

    let material = materials.add(StandardMaterial {
        base_color: GREEN_400.into(),
        ..default()
    });

    let mesh = meshes.add(Cuboid::from_length(BL));

    for i in (-10..=10) {
        for j in (-10..=10) {
            let x = i as f32 * BL;
            let y = j as f32 * BL;
            let offset = 5.;
            let depth = BL / 2.;
            let z = GAME_PLANE - offset - rand::random_range((0_f32..=depth)) - (BL * 0.5);

            let spawn_here = rand::random_bool(0.9);
            if !spawn_here {
                continue;
            }

            commands.spawn((
                MeshMaterial3d(material.clone()),
                Mesh3d(mesh.clone()),
                Transform::from_xyz(x, y, z),
            ));
        }
    }
}
