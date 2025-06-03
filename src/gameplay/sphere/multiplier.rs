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

fn on_hit(trigger: Trigger<OnCollisionStart>, collisions: Collisions) {
    info!("In multiplier on hit");
    let Some(contact_pair) = collisions.get(trigger.target(), trigger.collider) else {
        info!("no contact pair!");
        return;
    };

    // OPTION 1: Iterate over all contact manifolds and their points.
    // Iterate over the contact manifolds (kinda like contact surfaces).
    // For convex-convex contacts there's only one.
    for manifold in contact_pair.manifolds.iter() {
        // Iterate over contact points in the manifold.
        // For a circle or sphere there's only one.
        for manifold_point in manifold.points.iter() {
            // Use `global_point1()` and provide the translation and rotation
            // to get the point in global space.
            println!(
                "opt 1: Local point on `trigger.target` is {}",
                manifold_point.local_point1,
            );
        }
    }

    // OPTION 2: Just get the deepest contact point.
    if let Some(deepest_contact) = contact_pair.find_deepest_contact() {
        // Use `global_point1()` and provide the translation and rotation
        // to get the point in global space.
        println!(
            "opt 2: Local point on `trigger.target` is {}",
            deepest_contact.local_point1,
        );
    }
}
