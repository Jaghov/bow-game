use avian3d::prelude::*;
use bevy::prelude::*;

use super::Sphere;
use crate::third_party::avian3d::GameLayer;

#[derive(Component, Default)]
#[require(Sphere)]
pub struct Normal;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_normal);
}
fn insert_normal(trigger: Trigger<OnAdd, Normal>, mut commands: Commands) {
    info!("observed new normal insert");

    commands
        .entity(trigger.target())
        .insert((
            CollisionLayers::new(
                GameLayer::Sphere,
                [GameLayer::ArrowSensor, GameLayer::Sphere, GameLayer::Walls],
            ),
            Collider::sphere(1.),
            CollisionEventsEnabled,
        ))
        .observe(super::debug_collision)
        .observe(super::despawn_on_arrow_collision)
        .observe(super::despawn_on_bouncyball_collision)
        .observe(super::despawn_on_hit_by_explosion);
}
