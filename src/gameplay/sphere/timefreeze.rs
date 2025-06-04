use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    gameplay::{
        arrow::{Arrow, Canceled, NockedOn},
        sphere::{KeepOnCollideWith, SphereAssets, SphereType, sphere_defaults},
        timefreeze::FreezeTime,
    },
    third_party::avian3d::GameLayer,
};

pub fn timefreeze(assets: &SphereAssets) -> impl Bundle {
    (
        sphere_defaults(assets),
        (
            TimeFreeze,
            SphereType::TimeFreeze,
            Sensor,
            MeshMaterial3d(assets.time_freeze.clone()),
        ),
    )
}
/// Notice this remains if collided on arrow
#[derive(Component)]
#[require(KeepOnCollideWith = KeepOnCollideWith::Arrow)]
pub struct TimeFreeze;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_timefreeze);
}
fn insert_timefreeze(trigger: Trigger<OnAdd, TimeFreeze>, mut commands: Commands) {
    info!("observed new timefreeze insert");
    commands
        .spawn((
            CollisionLayers::new(GameLayer::Arrow, GameLayer::Arrow),
            Collider::sphere(1.),
            Sensor,
            CollisionEventsEnabled,
            ChildOf(trigger.target()),
        ))
        .observe(super::debug_collision)
        .observe(super::despawn_on_arrow)
        .observe(freeze_on_arrow_collision);

    commands
        .spawn((
            CollisionLayers::new(GameLayer::Sphere, GameLayer::Sphere),
            Collider::sphere(1.),
            CollisionEventsEnabled,
            ChildOf(trigger.target()),
        ))
        .observe(super::debug_collision);
}

fn freeze_on_arrow_collision(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    arrows: Query<Entity, (With<Arrow>, Without<Canceled>, Without<NockedOn>)>,
) {
    let event = trigger.event();
    let Ok(arrow) = arrows.get(event.collider) else {
        return;
    };
    info!("timefreeze collision: freezing time");
    commands.entity(arrow).despawn();
    commands.trigger(FreezeTime::new(trigger.target()));
}
