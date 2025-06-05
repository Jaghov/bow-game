use std::time::Duration;

use crate::gameplay::{GameSet, sphere::SphereAssets};

use super::Sphere;
use avian3d::prelude::*;
use bevy::{prelude::*, scene::SceneInstanceReady};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(add_destroyable_sphere).add_systems(
        Update,
        (
            tick_being_destroyed.in_set(GameSet::TickTimers),
            despawn_destroyed.in_set(GameSet::Update),
        ),
    );
    //todo
}

#[derive(Event)]
pub struct DestroySphere;

#[derive(Component)]
struct SphereGibScene(Handle<StandardMaterial>);

#[derive(Component)]
#[relationship(relationship_target = GibParent)]
pub struct GibsOf(Entity);

#[derive(Component)]
#[relationship_target(relationship = GibsOf)]
pub struct GibParent(Entity);

fn add_destroyable_sphere(
    trigger: Trigger<OnAdd, Sphere>,
    mut commands: Commands,
    materials: Query<&MeshMaterial3d<StandardMaterial>>,
    assets: Res<SphereAssets>,
) {
    let sphere_material = materials.get(trigger.target()).unwrap();

    commands
        .spawn((
            SceneRoot(assets.gibs.clone()),
            SphereGibScene(sphere_material.0.clone()),
            Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(1.3)),
            ChildOf(trigger.target()),
            GibsOf(trigger.target()),
            CollisionLayers::NONE,
            ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
            RigidBodyDisabled,
            Visibility::Hidden,
        ))
        .observe(ready_gibs);

    commands.entity(trigger.target()).observe(destroy_sphere);
}

fn ready_gibs(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    gibs: Query<&SphereGibScene>,
    meshes: Query<Entity, With<Mesh3d>>,
    children: Query<&Children>,
) {
    let gib = gibs.get(trigger.target()).unwrap();
    for child in children.iter_descendants(trigger.target()) {
        let Ok(mesh_entity) = meshes.get(child) else {
            continue;
        };
        commands
            .entity(mesh_entity)
            .insert(MeshMaterial3d(gib.0.clone()));
    }
}

#[derive(Component)]
struct BeingDestroyed(Timer);

// listener should ONLY be on the Sphere component.
fn destroy_sphere(
    trigger: Trigger<DestroySphere>,
    mut commands: Commands,
    gib_parents: Query<&GibParent>,
    destroying: Query<&BeingDestroyed>,
    mut gib_scenes: Query<&mut Visibility, With<SphereGibScene>>,
) {
    //todo: detach gibs from the sphere, make visible, attach rigid bodies to all the components
    // and apply a fake force in the area based on where the point of contact was for the collider.
    //
    // despawn sphere
    if destroying.get(trigger.target()).is_ok() {
        return;
    }

    commands
        .entity(trigger.target())
        //.remove::<(RigidBody, Mesh3d)>()
        .insert((
            Visibility::Hidden,
            RigidBodyDisabled,
            BeingDestroyed(Timer::new(Duration::from_secs(1), TimerMode::Once)),
        ));

    let gib_parent = gib_parents.get(trigger.target()).unwrap();
    commands.entity(gib_parent.0).remove::<RigidBodyDisabled>();
    let mut gib_visibility = gib_scenes.get_mut(gib_parent.0).unwrap();
    *gib_visibility = Visibility::Visible;
}
fn tick_being_destroyed(mut being_destroyed: Query<&mut BeingDestroyed>, time: Res<Time>) {
    for mut timer in &mut being_destroyed {
        timer.0.tick(time.delta());
    }
}
fn despawn_destroyed(mut commands: Commands, destroyed: Query<(Entity, &BeingDestroyed)>) {
    for (entity, timer) in &destroyed {
        if timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}
