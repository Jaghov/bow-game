use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    gameplay::sphere::{ShouldMultiply, Sphere, SphereType},
    third_party::avian3d::GameLayer,
    world::GAME_PLANE,
};

#[derive(Component)]
#[require(Sphere)]
pub struct GravitySphere;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_gravity_sphere);
}
fn insert_gravity_sphere(trigger: Trigger<OnAdd, GravitySphere>, mut commands: Commands) {
    commands
        .spawn((
            CollisionLayers::new(
                GameLayer::Sphere,
                [GameLayer::Arrow, GameLayer::Sphere, GameLayer::Walls],
            ),
            Collider::sphere(1.),
            Restitution::PERFECTLY_ELASTIC,
            CollisionEventsEnabled,
            ChildOf(trigger.target()),
        ))
        .observe(super::debug_collision);
}
