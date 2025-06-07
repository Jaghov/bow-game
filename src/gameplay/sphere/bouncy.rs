use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    gameplay::sphere::{Absorber, FromMultiply, ShouldMultiply, Sphere, SphereAssets},
    third_party::avian3d::GameLayer,
    world::GAME_PLANE,
};

#[derive(Component, Default)]
#[require(Sphere)]
pub struct Bouncy;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_bouncy);
}
fn insert_bouncy(
    trigger: Trigger<OnAdd, Bouncy>,
    absorbers: Query<(), With<Absorber>>,
    mut commands: Commands,
    assets: Res<SphereAssets>,
) {
    let mut commands = commands.entity(trigger.target());

    if absorbers.get(trigger.target()).is_err() {
        commands.insert((
            CollisionLayers::new(
                GameLayer::Sphere,
                [GameLayer::Arrow, GameLayer::Sphere, GameLayer::Walls],
            ),
            MeshMaterial3d(assets.bouncy.clone()),
        ));
    }

    commands
        .insert((
            Dominance(-1),
            Restitution::PERFECTLY_ELASTIC,
            Friction::ZERO,
        ))
        .observe(on_multiply)
        .observe(super::despawn_on_hit_by_explosion);
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
            Name::new("Bouncy Replica"),
            Bouncy,
            FromMultiply::default(),
            transform,
            LinearVelocity(velocity),
        ));
    }
}
