use avian3d::prelude::*;
use bevy::prelude::*;

use crate::gameplay::sphere::{
    SphereAssets, SphereType, despawn::BeginDespawning, sphere_defaults,
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
pub struct Multiplier;

fn insert_multiplier(trigger: Trigger<OnAdd, Multiplier>, mut commands: Commands) {
    info!("observed new normal insert");
    commands
        .entity(trigger.target())
        .observe(start_despawn)
        .observe(on_hit);
}

fn start_despawn(
    trigger: Trigger<BeginDespawning>,
    mut commands: Commands,
    multipliers: Query<Entity, With<Multiplier>>,
) {
    let multiplier = multipliers.get(trigger.target()).unwrap();
    commands.entity(multiplier).try_despawn();
}

/// An event that tells an observer to multiply with an array
/// of rotations relative to the observing entity's rotation
#[derive(Event)]
pub struct ShouldMultiply {
    pub location: Vec3,
    pub rot_offset: Vec<f32>,
}

fn on_hit(trigger: Trigger<OnCollisionStart>, mut commands: Commands, collisions: Collisions) {
    info!("In multiplier on hit");

    let contact_pair = match collisions.get(trigger.target(), trigger.collider) {
        Some(pair) => pair,
        None => match collisions.get(trigger.collider, trigger.target()) {
            Some(pair) => pair,
            None => {
                info!("no contact pair!");
                return;
            }
        },
    };
    let Some(deepest_contact) = contact_pair.find_deepest_contact() else {
        warn!("multiplier was hit, but couldn't find deepest contact point!");
        return;
    };
    commands.trigger_targets(
        ShouldMultiply {
            location: deepest_contact.local_point1,
            rot_offset: vec![35.0_f32.to_radians(), -35.0_f32.to_radians()],
        },
        trigger.collider,
    );
}
