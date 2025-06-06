use avian3d::prelude::*;
use bevy::prelude::*;

use super::Sphere;
use crate::{gameplay::sphere::SphereAssets, third_party::avian3d::GameLayer};

#[derive(Component, Default)]
#[require(Sphere)]
pub struct Normal;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_normal);
}
fn insert_normal(
    trigger: Trigger<OnAdd, Normal>,
    mut commands: Commands,
    assets: Res<SphereAssets>,
) {
    info!("observed new normal insert");

    commands
        .entity(trigger.target())
        .insert((
            MeshMaterial3d(assets.normal.clone()),
            CollisionLayers::new(
                GameLayer::Sphere,
                [GameLayer::ArrowSensor, GameLayer::Sphere, GameLayer::Walls],
            ),
        ))
        .observe(super::debug_collision)
        .observe(super::despawn_on_arrow_collision)
        .observe(super::despawn_on_bouncyball_collision)
        .observe(super::despawn_on_hit_by_explosion);
}
