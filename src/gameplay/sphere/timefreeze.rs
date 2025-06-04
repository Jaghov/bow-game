use avian3d::prelude::*;
use bevy::prelude::*;

use crate::gameplay::{
    arrow::{Arrow, Canceled, NockedIn},
    sphere::{
        KeepOnCollideWith, SphereAssets, SphereType, despawn::BeginDespawning, sphere_defaults,
    },
    timefreeze::FreezeTime,
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
        .entity(trigger.target())
        .observe(start_despawn)
        .observe(freeze_on_arrow_collision);
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

// ignore if the hit comes from an arrow
fn start_despawn(
    trigger: Trigger<BeginDespawning>,
    mut commands: Commands,
    normals: Query<Entity, With<TimeFreeze>>,
    arrows: Query<(), With<Arrow>>,
) {
    let normal = normals.get(trigger.target()).unwrap();
    commands.entity(normal).try_despawn();
}
