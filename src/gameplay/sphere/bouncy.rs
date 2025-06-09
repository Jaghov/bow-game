use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    gameplay::{
        arrow::{Arrow, NockedOn},
        level::Walls,
        sphere::{Absorber, FromMultiply, MustMark, ShouldMultiply, Sphere, SphereAssets},
    },
    third_party::avian3d::GameLayer,
    world::GAME_PLANE,
};

#[derive(Component, Default)]
#[require(Sphere)]
#[require(MustMark)]
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
        .insert((Restitution::PERFECTLY_ELASTIC, Friction::ZERO))
        .observe(on_multiply)
        .observe(despawn_arrow_on_contact)
        .observe(increase_velocity_on_collision)
        .observe(super::despawn_on_hit_by_explosion);
}

fn despawn_arrow_on_contact(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    absorbers: Query<(), With<Absorber>>,
    arrows: Query<(), (With<Arrow>, Without<NockedOn>)>,
    colliders: Query<&ColliderOf>,
) {
    if absorbers.get(trigger.target()).is_ok() {
        return;
    };

    let event = trigger.event();
    let Ok(collider) = colliders.get(event.collider) else {
        return;
    };
    if arrows.get(collider.body).is_err() {
        return;
    }
    commands.entity(collider.body).try_despawn();
}

fn increase_velocity_on_collision(
    trigger: Trigger<OnCollisionStart>,
    valid_colliders: Query<(), (Without<NockedOn>, Without<Walls>)>,
    colliders: Query<&ColliderOf>,
    mut velocity: Query<&mut LinearVelocity>,
) {
    let event = trigger.event();
    let Ok(collider) = colliders.get(event.collider) else {
        return;
    };
    if valid_colliders.get(collider.body).is_ok() {
        return;
    }

    let Ok(ball_collider) = colliders.get(trigger.target()) else {
        return;
    };

    let Ok(mut lvel) = velocity.get_mut(ball_collider.body) else {
        return;
    };

    lvel.0 *= 1.5;
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
