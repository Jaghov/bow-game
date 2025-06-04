use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    gameplay::sphere::{KeepOnCollideWith, SphereAssets, SphereType, sphere_defaults},
    third_party::avian3d::GameLayer,
};

pub fn multiplier(assets: &SphereAssets) -> impl Bundle {
    (
        sphere_defaults(assets),
        (
            Multiplier,
            SphereType::Multiplier,
            Sensor,
            MeshMaterial3d(assets.multiplier.clone()),
        ),
    )
}

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_multiplier);
}

#[derive(Component)]
#[require(KeepOnCollideWith = KeepOnCollideWith::NeverKeep)]
pub struct Multiplier;

fn insert_multiplier(trigger: Trigger<OnAdd, Multiplier>, mut commands: Commands) {
    info!("observed new multiplier insert");

    commands
        .spawn((
            CollisionLayers::new(GameLayer::Arrow, GameLayer::Arrow),
            Collider::sphere(1.),
            Sensor,
            CollisionEventsEnabled,
            ChildOf(trigger.target()),
        ))
        .observe(super::debug_collision)
        .observe(super::despawn_on_arrow)
        .observe(multiply_collider_on_hit);

    commands
        .spawn((
            CollisionLayers::new(GameLayer::Sphere, GameLayer::Sphere),
            Collider::sphere(1.),
            CollisionEventsEnabled,
            ChildOf(trigger.target()),
        ))
        .observe(super::debug_collision);
}

/// An event that tells an observer to multiply with an array
/// of rotations relative to the observing entity's rotation
#[derive(Event)]
pub struct ShouldMultiply {
    /// the point of contact relative to the observer's collider
    pub local_point: Vec3,
    pub rot_offset: Vec<f32>,
}

fn multiply_collider_on_hit(
    trigger: Trigger<OnCollisionStart>,
    transforms: Query<&Transform>,
    mut commands: Commands,
    collisions: Collisions,
) {
    info!("In multiplier on hit");

    // if point to use is true, use local point 2.
    // else, use 1.
    let Some(contact_pair) = collisions.get(trigger.target(), trigger.collider) else {
        info!("no contact pair!");
        return;
    };

    let Some(deepest_contact) = contact_pair.find_deepest_contact() else {
        warn!("multiplier was hit, but couldn't find deepest contact point!");
        return;
    };
    let hit_trns = transforms.get(trigger.target()).unwrap();

    let local_point = if contact_pair.collider2 == trigger.collider {
        deepest_contact.local_point1
    } else {
        deepest_contact.local_point2
    };

    commands.trigger_targets(
        ShouldMultiply {
            local_point: hit_trns.translation + local_point,
            rot_offset: vec![35.0_f32.to_radians(), -35.0_f32.to_radians()],
        },
        trigger.collider,
    );
}
