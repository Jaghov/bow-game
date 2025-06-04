use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    gameplay::{arrow::Arrow, sphere::Sphere},
    third_party::avian3d::GameLayer,
};

#[derive(Component)]
pub struct Bouncy;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_bouncy);
}
fn insert_bouncy(trigger: Trigger<OnAdd, Bouncy>, mut commands: Commands) {
    // todo: you figured out that going through the floor does weird things to
    // bouncy ball linear velocity
    commands
        .spawn((
            // CollisionLayers::new(GameLayer::ArrowSensors, GameLayer::ArrowSensors),
            // Collider::sphere(1.),
            // CollisionEventsEnabled,
            // ChildOf(trigger.target()),
        ))
        .observe(super::debug_collision)
        .observe(on_arrow_contact);

    commands
        .spawn((
            CollisionLayers::new(GameLayer::Sphere, [GameLayer::Sphere, GameLayer::Default]),
            Collider::sphere(1.),
            CollisionEventsEnabled,
            ChildOf(trigger.target()),
        ))
        .observe(super::debug_collision);
}

fn on_arrow_contact(
    trigger: Trigger<OnCollisionEnd>,
    arrows: Query<&LinearVelocity, (With<Arrow>, Without<Sphere>)>,
    mut spheres: Query<&mut LinearVelocity, (With<Sphere>, Without<Arrow>)>,
    colliders: Query<&ColliderOf>,
) {
    // let event = trigger.event();
    // let Ok(arrow_vel) = arrows.get(event.collider) else {
    //     return;
    // };

    // let Ok(collider) = colliders.get(trigger.target()) else {
    //     return;
    // };

    // let Ok(mut sphere) = spheres.get_mut(collider.body) else {
    //     warn!("bouncy ball wasn't a sphere?");
    //     return;
    // };

    // let add_to_velocity = arrow_vel.0 * 2.;
    // sphere.0 += add_to_velocity;
}
