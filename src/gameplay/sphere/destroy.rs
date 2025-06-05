use std::time::Duration;

use crate::{
    gameplay::{GameSet, sphere::SphereAssets},
    third_party::avian3d::GameLayer,
};

use super::Sphere;
use avian3d::prelude::*;
use bevy::{prelude::*, scene::SceneInstanceReady};

/*

New plan:
- Once the resource has been loaded, we're going to spawn a scene instance
of the gibs. When it's ready, we take all of the mesh handles and transforms, and store
them in a resource.
When a sphere is destroyed, we clone these meshes, transforms, etc. and apply it to
the position of the despawned sphere. ez.

*/

pub(super) fn plugin(app: &mut App) {
    app.add_observer(add_destroyable_sphere).add_systems(
        Update,
        (
            tick_being_destroyed.in_set(GameSet::TickTimers),
            (
                //ready_gib_bodies,
                //update_gib_transforms,
                despawn_destroyed,
                //realize_gib_explosion,
            )
                .in_set(GameSet::Update),
        ),
    );
}

#[derive(Event)]
pub struct DestroySphere;

#[derive(Component)]
#[relationship(relationship_target = GibChild)]
pub struct GibsOf(Entity);

#[derive(Component)]
#[relationship_target(relationship = GibsOf)]
pub struct GibChild(Entity);

fn add_destroyable_sphere(trigger: Trigger<OnAdd, Sphere>, mut commands: Commands) {
    commands.entity(trigger.target()).observe(destroy_sphere);
}

#[derive(Component)]
struct BeingDestroyed(Timer);

// listener should ONLY be on the Sphere component.
fn destroy_sphere(trigger: Trigger<DestroySphere>, mut commands: Commands) {
    commands.entity(trigger.target()).try_despawn();
}
fn tick_being_destroyed(mut being_destroyed: Query<&mut BeingDestroyed>, time: Res<Time>) {
    for mut timer in &mut being_destroyed {
        timer.0.tick(time.delta());
    }
}
fn despawn_destroyed(mut commands: Commands, destroyed: Query<(Entity, &BeingDestroyed)>) {
    for (entity, timer) in &destroyed {
        if timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}
