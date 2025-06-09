use std::time::Duration;

use avian3d::prelude::*;
use bevy::prelude::*;

use super::Sphere;
use crate::{
    gameplay::{
        GameSet,
        sphere::{Absorber, DestroySphere, Exploder, HitByExplosion, LightFuse, SphereAssets},
    },
    third_party::avian3d::GameLayer,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_multiplier)
        .add_systems(
            Update,
            (|mut timers: Query<&mut FromAbsorberMultiply>, time: Res<Time>| {
                for mut timer in &mut timers {
                    timer.0.tick(time.delta());
                }
            })
            .in_set(GameSet::TickTimers),
        )
        .add_systems(
            PostUpdate,
            |mut commands: Commands, timers: Query<(Entity, &FromAbsorberMultiply)>| {
                for (entity, timer) in timers {
                    if timer.0.finished() {
                        commands.entity(entity).remove::<FromAbsorberMultiply>();
                    }
                }
            },
        );
}

#[derive(Component, Default)]
#[require(Sphere)]
pub struct Multiplier;

fn insert_multiplier(
    trigger: Trigger<OnAdd, Multiplier>,
    absorbers: Query<(), With<Absorber>>,
    mut commands: Commands,
    assets: Res<SphereAssets>,
) {
    let mut commands = commands.entity(trigger.target());

    if absorbers.get(trigger.target()).is_err() {
        commands.insert((
            MeshMaterial3d(assets.multiplier.clone()),
            CollisionLayers::new(
                GameLayer::Sphere,
                [GameLayer::ArrowSensor, GameLayer::Sphere],
            ),
        ));
    }

    commands
        .observe(super::despawn_on_arrow_collision)
        .observe(super::despawn_on_bouncyball_collision)
        .observe(multiply_collider_on_hit)
        .observe(multiply_explosion);
}

/// adds a cooldown to something multiplied so it won't multiply forever.
#[derive(Component)]
pub struct FromAbsorberMultiply(Timer);
impl FromAbsorberMultiply {
    pub fn forever() -> Self {
        Self(Timer::new(Duration::MAX, TimerMode::Once))
    }
}
impl Default for FromAbsorberMultiply {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs(1), TimerMode::Once))
    }
}

fn multiply_explosion(
    trigger: Trigger<HitByExplosion>,
    absorbers: Query<(), With<Absorber>>,
    mut commands: Commands,
    transforms: Query<&Transform>,
) {
    let Ok(location) = transforms.get(trigger.target()) else {
        return;
    };
    let explosion_location = trigger.event().location();

    let diff = (location.translation.xy() - explosion_location).normalize_or_zero();

    let z_rot = -diff.x.atan2(diff.y);

    let rotation = Quat::from_rotation_z(z_rot);

    if absorbers.get(trigger.target()).is_ok() {
        // this will stop infinite explosions
        if trigger.event().was_from_multiple() {
            return;
        }
    }

    for rotation_offset in [70.0_f32.to_radians(), 0., -70.0_f32.to_radians()] {
        let rotation = rotation * Quat::from_rotation_z(rotation_offset);

        let offset = rotation * Vec3::new(0., 6., 0.);

        let translation = location.translation + offset;

        let transform = Transform::from_translation(translation).with_rotation(rotation);
        commands
            .spawn((
                Name::new("Exploder Replica"),
                Exploder,
                Sensor,
                FromAbsorberMultiply::forever(),
                transform,
            ))
            .trigger(LightFuse(3));
    }
    let Ok(mut entity) = commands.get_entity(trigger.target()) else {
        return;
    };
    entity.trigger(DestroySphere);
}

/// An event that tells an observer to multiply with an array
/// of rotations relative to the observing entity's rotation
///
/// NOTE: make sure you add `FromMultiply` to your duplicate!!
#[derive(Event)]
pub struct ShouldMultiply {
    pub is_from_absorber: bool,
    /// the point of contact relative to the observer's collider
    pub local_point: Vec3,
    pub rot_offset: Vec<f32>,
}

fn multiply_collider_on_hit(
    trigger: Trigger<OnCollisionStart>,
    absorbers: Query<(), With<Absorber>>,
    already_hit: Query<(), With<FromAbsorberMultiply>>,
    transforms: Query<&GlobalTransform>,
    mut commands: Commands,
    colliders: Query<&ColliderOf>,
    collisions: Collisions,
) {
    let Some(contact_pair) = collisions.get(trigger.target(), trigger.collider) else {
        info!("no contact pair!");
        return;
    };

    let Ok(collider) = colliders.get(trigger.collider) else {
        return;
    };

    if already_hit.get(collider.body).is_ok() {
        return;
    }

    let Some(deepest_contact) = contact_pair.find_deepest_contact() else {
        warn!("multiplier was hit, but couldn't find deepest contact point!");
        return;
    };
    let Ok(hit_trns) = transforms.get(trigger.target()) else {
        return;
    };

    let local_point = if contact_pair.collider2 == trigger.collider {
        deepest_contact.local_point1
    } else {
        deepest_contact.local_point2
    };

    let is_from_absorber = colliders
        .get(trigger.target())
        .is_ok_and(|c| absorbers.get(c.body).is_ok());

    commands.trigger_targets(
        ShouldMultiply {
            is_from_absorber,
            local_point: hit_trns.translation() + local_point,
            rot_offset: vec![35.0_f32.to_radians(), -35.0_f32.to_radians()],
        },
        collider.body,
    );
}
