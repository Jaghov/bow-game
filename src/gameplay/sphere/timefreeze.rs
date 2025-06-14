use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    gameplay::{
        arrow::{Arrow, Canceled, NockedOn},
        sphere::{Sphere, SphereAssets},
        timefreeze::FreezeTime,
    },
    third_party::avian3d::GameLayer,
};

/// Notice this remains if collided on arrow
#[derive(Component, Default)]
#[require(Sphere)]
pub struct TimeFreeze;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_timefreeze);
}
fn insert_timefreeze(
    trigger: Trigger<OnAdd, TimeFreeze>,
    mut commands: Commands,
    assets: Res<SphereAssets>,
) {
    commands
        .entity(trigger.target())
        .insert((
            CollisionLayers::new(
                GameLayer::Sphere,
                [GameLayer::ArrowSensor, GameLayer::Sphere],
            ),
            MeshMaterial3d(assets.time_freeze.clone()),
        ))
        .observe(freeze_on_arrow_collision);
}

fn freeze_on_arrow_collision(
    trigger: Trigger<OnCollisionStart>,
    colliders: Query<&ColliderOf>,
    mut commands: Commands,
    arrows: Query<Entity, (With<Arrow>, Without<Canceled>, Without<NockedOn>)>,
) {
    let event = trigger.event();

    let Ok(collider) = colliders.get(event.collider) else {
        return;
    };
    let Ok(arrow) = arrows.get(collider.body) else {
        return;
    };
    let Ok(t_collider) = colliders.get(trigger.target()) else {
        return;
    };
    info!("timefreeze collision: freezing time");
    commands.entity(arrow).despawn();
    commands.trigger(FreezeTime::new(t_collider.body));
}
