use avian3d::prelude::{
    ColliderConstructor, ColliderConstructorHierarchy, GravityScale, RigidBody,
};
use bevy::prelude::*;

use std::f32::consts::FRAC_PI_2;

use super::WALL;
use crate::{
    Screen,
    gameplay::{arrow::ArrowAssets, bow::BowAssets},
    world::backdrop::{BACKDROP_OFFSET, BLOCK_LEN},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn_items)
        .add_systems(Update, set_locations.run_if(in_state(Screen::Title)));
    //todo
}

// note these are different from the game components
#[derive(Component)]
struct Arrow(usize);

#[derive(Component)]
struct Bow;

fn spawn_items(mut commands: Commands, bow_assets: Res<BowAssets>, arrow_assets: Res<ArrowAssets>) {
    commands.spawn((
        Bow,
        StateScoped(Screen::Title),
        Transform::from_xyz(
            BLOCK_LEN * 7. - 2.8,
            BLOCK_LEN * 4. + 1.2,
            -1.3 - BACKDROP_OFFSET,
        )
        .with_rotation(Quat::from_euler(EulerRot::XYX, FRAC_PI_2, FRAC_PI_2, -0.2))
        .with_scale(Vec3::splat(0.5)),
        //RigidBody::Dynamic,
        //ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh),
        GravityScale(3.),
        SceneRoot(bow_assets.scene.clone()),
    ));
    for i in (0..5) {
        commands.spawn((
            Arrow(i),
            StateScoped(Screen::Title),
            SceneRoot(arrow_assets.glowing.clone()),
        ));
    }

    //todo
}

#[cfg_attr(feature = "hot", bevy_simple_subsecond_system::prelude::hot)]
fn set_locations(
    mut bow: Query<&mut Transform, With<Bow>>,
    mut arrows: Query<(&mut Transform, &Arrow), Without<Bow>>,
) {
    let mut bow = bow.single_mut().unwrap();

    *bow = Transform::from_xyz(
        BLOCK_LEN * 7. - 2.85,
        BLOCK_LEN * 4. + 1.2,
        -1.5 - BACKDROP_OFFSET,
    )
    .with_rotation(Quat::from_euler(
        EulerRot::XYX,
        FRAC_PI_2 - 0.15,
        FRAC_PI_2 + 0.02,
        -0.2,
    ))
    .with_scale(Vec3::splat(0.5));

    for (mut arrow_trns, arrow) in &mut arrows {
        let offset = arrow.0 as f32 * 0.2;
        *arrow_trns = Transform::from_xyz(
            BLOCK_LEN * 7. - 2.83,
            BLOCK_LEN * 4. + 0.7 - offset,
            -1.65 - BACKDROP_OFFSET,
        )
        .with_rotation(Quat::from_euler(
            EulerRot::XYX,
            FRAC_PI_2 - 0.15,
            FRAC_PI_2 + 0.02,
            -0.2,
        ))
        .with_scale(Vec3::splat(0.5));
    }

    //todod
}
