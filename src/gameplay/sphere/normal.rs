use avian3d::prelude::{CollisionStarted, OnCollisionEnd, OnCollisionStart};
use bevy::prelude::*;

use crate::gameplay::{
    arrow::Arrow,
    sphere::{KeepOnCollide, Normal, SphereAssets, SphereType, sphere_defaults},
};

pub fn normal(assets: &SphereAssets) -> impl Bundle {
    (
        sphere_defaults(assets),
        (
            Normal,
            SphereType::Normal,
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
        .observe(observe_collisions);
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
