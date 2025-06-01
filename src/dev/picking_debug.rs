use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::screens::Screen;

use super::assets::PickMaterials;

pub fn plugin(app: &mut App) {
    app.add_observer(observe_pickables);

    app.add_systems(OnEnter(Screen::Gameplay), spawn_simple_pickable_mesh);
}

#[derive(Component, Reflect)]
pub struct PickableMesh;

fn observe_pickables(
    trigger: Trigger<OnAdd, PickableMesh>,
    mut commands: Commands,
    materials: Res<PickMaterials>,
) {
    info!("Observing pickable!");
    commands
        .entity(trigger.target())
        .observe(update_material_on::<Pointer<Over>>(
            materials.hover_matl.clone(),
        ))
        .observe(update_material_on::<Pointer<Out>>(
            materials.white_matl.clone(),
        ))
        .observe(update_material_on::<Pointer<Pressed>>(
            materials.pressed_matl.clone(),
        ))
        .observe(update_material_on::<Pointer<Released>>(
            materials.hover_matl.clone(),
        ));
}

/// Returns an observer that updates the entity's material to the one specified.
fn update_material_on<E>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(Trigger<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |trigger, mut query| {
        if let Ok(mut material) = query.get_mut(trigger.target()) {
            material.0 = new_material.clone();
        }
    }
}

const SHAPES_X_EXTENT: f32 = 12.0;
const Z_EXTENT: f32 = 5.0;

fn spawn_simple_pickable_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<PickMaterials>,
) {
    commands.spawn((
        RigidBody::Kinematic,
        PickableMesh,
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.white_matl.clone()),
        Transform::from_xyz(-SHAPES_X_EXTENT / 2., 2.0, Z_EXTENT / 2.)
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
        Collider::from(Cuboid::default()),
    ));
}
