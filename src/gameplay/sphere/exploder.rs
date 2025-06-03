use avian3d::prelude::Sensor;
use bevy::prelude::*;

use crate::gameplay::sphere::{
    BeginDespawning, KeepOnCollideWith, SphereAssets, SphereType, sphere_defaults,
};

pub fn exploder(assets: &SphereAssets) -> impl Bundle {
    (
        sphere_defaults(assets),
        (
            Exploder,
            SphereType::Exploder,
            Sensor,
            MeshMaterial3d(assets.exploder.clone()),
        ),
    )
}

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_exploder);
    //todo
}

#[derive(Component)]
#[require(KeepOnCollideWith = KeepOnCollideWith::Sphere)]
pub struct Exploder;

fn insert_exploder(trigger: Trigger<OnAdd, Exploder>, mut commands: Commands) {
    info!("observed new normal insert");
    commands.entity(trigger.target()).observe(start_despawn);
}

fn start_despawn(
    trigger: Trigger<BeginDespawning>,
    mut commands: Commands,
    exploders: Query<Entity, With<Exploder>>,
) {
    let exploder = exploders.get(trigger.target()).unwrap();
    commands.entity(exploder).try_despawn();
}
