use avian3d::prelude::*;
use bevy::prelude::*;

use super::Sphere;
use crate::gameplay::{
    GameSet,
    arrow::{Arrow, Canceled},
    sphere::KeepOnCollideWith,
};

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
    arrows: Query<(), (With<Arrow>, Without<Canceled>)>,
    spheres: Query<(Entity, &KeepOnCollideWith), (With<Sphere>, Without<DespawnStarted>)>,
) {
    for CollisionStarted(entity1, entity2) in collision_events.read() {
        let ((sphere, keep_if), other) = match spheres.get(*entity1) {
            Ok(res) => (res, *entity2),
            Err(_) => match spheres.get(*entity2) {
                Ok(res) => (res, *entity1),
                Err(_) => continue,
            },
        };

        match keep_if {
            KeepOnCollideWith::Arrow => {
                // dont do anything if colliding with an arrow, or the other collider isn't a sphere.
                if arrows.get(other).is_ok() {
                    continue;
                }
                if spheres.get(other).is_err() {
                    continue;
                }
            }
            KeepOnCollideWith::Sphere => {
                //dont do anything if colliding with a sphere or non-arrows
                if spheres.get(other).is_ok() {
                    continue;
                }
                if arrows.get(other).is_err() {
                    continue;
                }
            }
            KeepOnCollideWith::Both => continue,
            KeepOnCollideWith::NeverKeep => {
                if spheres.get(other).is_err() && arrows.get(other).is_err() {
                    continue;
                }
            }
        }

        info!("sphere collided!");

        commands.entity(sphere).insert(DespawnStarted);

        // note that observers should assume they must now despawn!
        commands.trigger_targets(BeginDespawning, sphere);
    }
    //todo
}
