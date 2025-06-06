use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    gameplay::sphere::{FromMultiply, ShouldMultiply, Sphere, SphereType},
    third_party::avian3d::GameLayer,
    world::GAME_PLANE,
};
/// WIP: need to fix a few systems
#[derive(Component, Default)]
#[require(Sphere)]
pub struct Bouncy;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_bouncy);
}
fn insert_bouncy(trigger: Trigger<OnAdd, Bouncy>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert((
            CollisionLayers::new(
                GameLayer::Sphere,
                [GameLayer::Arrow, GameLayer::Sphere, GameLayer::Walls],
            ),
            Collider::sphere(1.),
            Restitution::PERFECTLY_ELASTIC,
            CollisionEventsEnabled,
        ))
        .observe(super::debug_collision);
    commands.entity(trigger.target()).observe(on_multiply);
}

fn on_multiply(
    trigger: Trigger<ShouldMultiply>,
    mut commands: Commands,
    bouncy_balls: Query<(&Transform, &LinearVelocity), With<Bouncy>>,
) {
    info!("in bouncy on multiply");
    let event = trigger.event();
    let Ok((transform, lvel)) = bouncy_balls.get(trigger.target()) else {
        warn!("Bouncy ball was commanded to multiply, but its required components were not found!");
        return;
    };

    let multiply_origin = event.local_point.with_z(GAME_PLANE);

    for rotation_offset in &event.rot_offset {
        let quatrot = Quat::from_rotation_z(*rotation_offset);
        let rotation = transform.rotation * Quat::from_rotation_z(*rotation_offset);

        let velocity = quatrot * lvel.0;
        let offset = velocity.normalize_or_zero() * 2.2;

        let transform = Transform::from_translation(multiply_origin + offset)
            .with_rotation(rotation)
            .with_scale(transform.scale);

        commands.spawn((
            SphereType::Bouncy,
            FromMultiply::default(),
            transform,
            LinearVelocity(velocity),
        ));
    }
}
