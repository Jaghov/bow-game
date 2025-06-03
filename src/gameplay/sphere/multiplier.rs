use avian3d::prelude::Sensor;
use bevy::prelude::*;

use crate::gameplay::sphere::{
    Normal, SphereAssets, SphereType, despawn::BeginDespawning, sphere_defaults,
};

pub fn multiplier(assets: &SphereAssets) -> impl Bundle {
    (
        sphere_defaults(assets),
        (
            Multiplier,
            SphereType::Multiplier,
            Sensor,
            MeshMaterial3d(assets.multiplier.clone()),
        ),
    )
}

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_multiplier);
}

#[derive(Component)]
pub struct Multiplier;

fn insert_multiplier(trigger: Trigger<OnAdd, Normal>, mut commands: Commands) {
    info!("observed new normal insert");
    commands.entity(trigger.target()).observe(on_hit);
}

fn on_hit(
    trigger: Trigger<BeginDespawning>,
    mut commands: Commands,
    normals: Query<Entity, With<Normal>>,
) {
    let normal = normals.get(trigger.target()).unwrap();
    commands.entity(normal).try_despawn();
}
