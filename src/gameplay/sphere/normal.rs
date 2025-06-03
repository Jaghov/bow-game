use avian3d::prelude::*;
use bevy::prelude::*;

use crate::gameplay::sphere::{
    SphereAssets, SphereType, despawn::BeginDespawning, sphere_defaults,
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
#[derive(Component)]
pub struct Normal;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_normal);
}
fn insert_normal(trigger: Trigger<OnAdd, Normal>, mut commands: Commands) {
    info!("observed new normal insert");
    commands.entity(trigger.target()).observe(start_despawn);
}

fn start_despawn(
    trigger: Trigger<BeginDespawning>,
    mut commands: Commands,
    normals: Query<Entity, With<Normal>>,
) {
    let normal = normals.get(trigger.target()).unwrap();
    commands.entity(normal).try_despawn();
}
