use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{gameplay::arrow::Arrow, third_party::avian3d::GameLayer};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(add_arrow_sensor);
}
#[derive(Event)]
pub(super) struct ArrowSensor;
fn add_arrow_sensor(trigger: Trigger<ArrowSensor>, mut commands: Commands) {
    info!("adding arrow sensor");
    commands
        .spawn((
            CollisionLayers::new(GameLayer::Arrow, GameLayer::Arrow),
            Collider::sphere(1.),
            Sensor,
            CollisionEventsEnabled,
            ChildOf(trigger.target()),
        ))
        .observe(observe_collision);
}

pub fn observe_collision(trigger: Trigger<OnCollisionStart>, arrows: Query<&Arrow>) {
    let event = trigger.event();

    info!(
        "Collision event: was arrow? {}",
        arrows.get(event.collider).is_ok()
    );
}
