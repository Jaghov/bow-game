use avian3d::prelude::*;
use bevy::prelude::*;

use super::Sphere;
use crate::gameplay::{GameSet, arrow::Arrow, sphere::KeepOnCollide};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, check_sphere_despawn.in_set(GameSet::Update));
}

// to filter out future events
#[derive(Component)]
struct DespawnStarted;

/// because things will need *time* to despawn, this is sent to the
/// sphere's observer so it can handle it.
#[derive(Event)]
pub struct BeginDespawning;

// this will trigger a despawn event for any spheres that need to be triggered when getting hit
fn check_sphere_despawn(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionStarted>,
    //todo: this may need to be something like `SphereTriggerer`
    arrow: Query<&Arrow>,
    spheres: Query<
        Entity,
        (
            With<Sphere>,
            Without<KeepOnCollide>,
            Without<DespawnStarted>,
        ),
    >,
) {
    for CollisionStarted(entity1, entity2) in collision_events.read() {
        let (_arrow, maybe_sphere) = match arrow.get(*entity1) {
            Ok(arrow) => (arrow, entity2),
            Err(_) => match arrow.get(*entity2) {
                Ok(arrow) => (arrow, entity1),
                Err(_) => continue,
            },
        };
        let Ok(sphere) = spheres.get(*maybe_sphere) else {
            continue;
        };
        info!("sphere and arrow collided!");

        commands.entity(sphere).insert(DespawnStarted);

        // note that observers should assume they must now despawn!
        commands.trigger_targets(BeginDespawning, sphere);
    }
    //todo
}
