use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    gameplay::{
        arrow::{Arrow, Canceled, NockedOn},
        sphere::Sphere,
        timefreeze::FreezeTime,
    },
    third_party::avian3d::GameLayer,
};

/// Notice this remains if collided on arrow
#[derive(Component)]
#[require(Sphere)]
pub struct TimeFreeze;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_timefreeze);
}
fn insert_timefreeze(trigger: Trigger<OnAdd, TimeFreeze>, mut commands: Commands) {
    info!("observed new timefreeze insert");
    commands
        .spawn((
            CollisionLayers::new(GameLayer::ArrowSensor, GameLayer::ArrowSensor),
            Collider::sphere(1.),
            Sensor,
            CollisionEventsEnabled,
            ChildOf(trigger.target()),
        ))
        .observe(super::debug_collision)
        .observe(freeze_on_arrow_collision);
}

fn freeze_on_arrow_collision(
    trigger: Trigger<OnCollisionStart>,
    colliders: Query<&ColliderOf>,
    mut commands: Commands,
    arrows: Query<Entity, (With<Arrow>, Without<Canceled>, Without<NockedOn>)>,
) {
    let event = trigger.event();
    let Ok(arrow) = arrows.get(event.collider) else {
        return;
    };
    info!("timefreeze collision: freezing time");
    commands.entity(arrow).despawn();
    commands.trigger(FreezeTime::new(
        colliders.get(trigger.target()).unwrap().body,
    ));
}
