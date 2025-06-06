use avian3d::prelude::*;
use bevy::prelude::*;

use super::Sphere;
use crate::{
    gameplay::sphere::{DestroySphere, HitByExplosion, LightFuse, SphereType},
    third_party::avian3d::GameLayer,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_multiplier);
}

#[derive(Component)]
#[require(Sphere)]
pub struct Multiplier;

fn insert_multiplier(trigger: Trigger<OnAdd, Multiplier>, mut commands: Commands) {
    info!("observed new multiplier insert");

    commands
        .entity(trigger.target())
        .insert((
            CollisionLayers::new(
                GameLayer::Sphere,
                [GameLayer::ArrowSensor, GameLayer::Sphere],
            ),
            Collider::sphere(1.),
            Sensor,
            CollisionEventsEnabled,
        ))
        .observe(super::debug_collision)
        .observe(super::despawn_on_arrow_collision)
        .observe(super::despawn_on_bouncyball_collision)
        .observe(multiply_collider_on_hit)
        .observe(multiply_explosion);
}

fn multiply_explosion(
    trigger: Trigger<HitByExplosion>,
    mut commands: Commands,
    transforms: Query<&Transform>,
) {
    let Ok(location) = transforms.get(trigger.target()) else {
        return;
    };
    let explosion_location = trigger.event().location();

    let diff = (location.translation.xy() - explosion_location).normalize();

    let z_rot = -diff.x.atan2(diff.y);

    let rotation = Quat::from_rotation_z(z_rot);

    for rotation_offset in [70.0_f32.to_radians(), 0., -70.0_f32.to_radians()] {
        let rotation = rotation * Quat::from_rotation_z(rotation_offset);

        let offset = rotation * Vec3::new(0., 6., 0.);

        let translation = location.translation + offset;

        let transform = Transform::from_translation(translation).with_rotation(rotation);
        commands
            .spawn((SphereType::Exploder, transform))
            .trigger(LightFuse(3));
    }
    commands.trigger_targets(DestroySphere, trigger.target());
}

/// An event that tells an observer to multiply with an array
/// of rotations relative to the observing entity's rotation
#[derive(Event)]
pub struct ShouldMultiply {
    /// the point of contact relative to the observer's collider
    pub local_point: Vec3,
    pub rot_offset: Vec<f32>,
}

#[derive(Component)]
struct AlreadyHit;

fn multiply_collider_on_hit(
    trigger: Trigger<OnCollisionStart>,
    already_hit: Query<&AlreadyHit>,
    transforms: Query<&GlobalTransform>,
    mut commands: Commands,
    colliders: Query<&ColliderOf>,
    collisions: Collisions,
) {
    info!("In multiplier on hit");
    if already_hit.get(trigger.target()).is_ok() {
        return;
    }

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

    info!("\n\ntriggering should_multiply!");
    commands.trigger_targets(
        ShouldMultiply {
            local_point: hit_trns.translation() + local_point,
            rot_offset: vec![35.0_f32.to_radians(), -35.0_f32.to_radians()],
        },
        colliders.get(trigger.collider).unwrap().body,
    );
}
