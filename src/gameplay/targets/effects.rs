use avian3d::{math::PI, prelude::*};
use bevy::prelude::*;

use crate::gameplay::{
    GameSet,
    arrow::{Arrow, ArrowAssets, Fired, STRENGTH_MULT},
};

use super::{Multiplier, Target};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        split_arrow_on_multiplier_collision.in_set(GameSet::Update),
    );
}

const SPLIT_ANGLE: f32 = PI / 3.;
const SPLIT_OFFSET: f32 = 2.0;
fn split_arrow_on_multiplier_collision(
    arrows: Query<(&Transform), With<Arrow>>,
    targets: Query<(&Transform), (With<Multiplier>, With<Target>)>,
    mut collisions: EventReader<CollisionStarted>,
    mut commands: Commands,
    assets: Res<ArrowAssets>,
) {
    collisions.read().for_each(|collision| {
        // check for Multiplier x Arrow Collision
        let (Ok(arrow_transform), Ok(target_transform)) = (
            arrows.get(collision.0).or(arrows.get(collision.1)),
            targets.get(collision.0).or(targets.get(collision.1)),
        ) else {
            return;
        };
        let mut left_transform = arrow_transform.with_translation(target_transform.translation);
        let mut right_transform = arrow_transform.with_translation(target_transform.translation);

        left_transform.rotate_local_z(SPLIT_ANGLE);
        let left = left_transform.up().normalize();
        right_transform.rotate_local_z(-SPLIT_ANGLE);
        let right = right_transform.up().normalize();

        left_transform.translation += left * SPLIT_OFFSET;
        right_transform.translation += right * SPLIT_OFFSET;
        // TODO use Arrow Spawn APIs

        let arrow_base = (Arrow, Fired, SceneRoot(assets.glowing.clone()));

        // Spawn Arrow 1
        commands.spawn((
            arrow_base.clone(),
            left_transform,
            LinearVelocity(left * STRENGTH_MULT),
        ));
        commands.spawn((
            arrow_base,
            right_transform,
            LinearVelocity(right * STRENGTH_MULT),
        ));

        // TODO add despawn marker to clean up components in schedule later
        // Despawn Old Arrow and Target
        commands.entity(collision.0).despawn();
        commands.entity(collision.1).despawn();
    });
}
