use avian3d::prelude::{OnCollisionStart, Sensor};
use bevy::prelude::*;

use crate::gameplay::{
    arrow::Arrow,
    sphere::{Hit, KeepOnCollide, Normal, SphereAssets, SphereType, sphere_defaults},
};

pub fn normal(assets: &SphereAssets) -> impl Bundle {
    (
        sphere_defaults(assets),
        (
            Normal,
            SphereType::Normal,
            Sensor,
            MeshMaterial3d(assets.normal.clone()),
        ),
    )
}

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_normal);
    //todo
}
fn insert_normal(trigger: Trigger<OnAdd, Normal>, mut commands: Commands) {
    info!("observed new normal insert");
    commands
        .entity(trigger.target())
        .observe(observe_collisions)
        .observe(on_hit);
}

fn observe_collisions(
    trigger: Trigger<OnCollisionStart>,
    arrow: Query<&Arrow>,
    remove: Query<&KeepOnCollide>,
) {
    let event = trigger.event();
    let Some(body) = event.body else {
        warn!("Collided with something without a body");
        return;
    };

    let Ok(arrow) = arrow.get(event.collider) else {
        info!("collider is arrow");
        return;
    };

    // let Ok(arrow) = arrow.get(body) else {
    //     warn!("collided with non-arrow");
    //     return;
    // };

    info!("normal ball collision with arrow");
}

fn on_hit(trigger: Trigger<Hit>, mut commands: Commands, normals: Query<Entity, With<Normal>>) {
    let normal = normals.get(trigger.target()).unwrap();
    commands.entity(normal).try_despawn();
}
