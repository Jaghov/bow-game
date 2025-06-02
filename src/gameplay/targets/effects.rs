use avian3d::prelude::{CollisionStarted, LinearVelocity};
use bevy::prelude::*;

use crate::gameplay::{
    GameSet,
    arrow::{Arrow, ArrowAssets, Fired},
};

use super::{Multiplier, Target};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        split_arrow_on_multiplier_collision.in_set(GameSet::Update),
    );
}

fn split_arrow_on_multiplier_collision(
    arrows: Query<(&LinearVelocity), With<Arrow>>,
    targets: Query<(&Transform), (With<Multiplier>, With<Target>)>,
    mut collisions: EventReader<CollisionStarted>,
    mut commands: Commands,
    assets: Res<ArrowAssets>,
) {
    collisions.read().for_each(|collision| {
        let (Ok(arrow_velocity), Ok(target_transform)) = (
            arrows.get(collision.0).or(arrows.get(collision.1)),
            targets.get(collision.0).or(targets.get(collision.1)),
        ) else {
            return;
        };

        // TODO use Arrow Spawn APIs

        // Spawn Arrow 1
        commands.spawn((
            Arrow,
            Fired,
            arrow_velocity.clone(),
            target_transform.clone(),
            SceneRoot(assets.glowing.clone()),
        ));

        // //Spawn Arrow 2
        // commands.spawn((
        //     Arrow,
        //     Fired,
        //     arrow_velocity.clone().refract(normal, eta),
        //     target_transform.clone(),
        //     SceneRoot(assets.glowing.clone()),
        // ));

        // TODO add despawn marker to clean up components in schedule later
        commands.entity(collision.0).despawn();
        commands.entity(collision.1).despawn();
    });
}
