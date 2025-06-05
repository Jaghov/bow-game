use std::time::Duration;

use crate::{
    gameplay::{GameSet, sphere::SphereAssets},
    third_party::avian3d::GameLayer,
};

use super::Sphere;
use avian3d::prelude::*;
use bevy::{prelude::*, scene::SceneInstanceReady};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(add_destroyable_sphere).add_systems(
        Update,
        (
            tick_being_destroyed.in_set(GameSet::TickTimers),
            (
                //ready_gib_bodies,
                //update_gib_transforms,
                despawn_destroyed,
                //realize_gib_explosion,
            )
                .in_set(GameSet::Update),
        ),
    );
    //.add_systems(Update, update_gib_transforms);
    //todo
}

#[derive(Event)]
pub struct DestroySphere;

#[derive(Component)]
struct SphereGibScene(Handle<StandardMaterial>);

#[derive(Component)]
#[relationship(relationship_target = GibChild)]
pub struct GibsOf(Entity);

#[derive(Component)]
#[relationship_target(relationship = GibsOf)]
pub struct GibChild(Entity);

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
            Transform::default(),
            GibsOf(trigger.target()),
            RigidBody::Dynamic,
            RigidBodyDisabled,
            ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        ))
        .observe(when_gib_scene_is_ready);

    commands.entity(trigger.target()).observe(destroy_sphere);
}

#[derive(Component)]
struct BeingDestroyed(Timer);

#[derive(Component)]
struct GibReady;

fn when_gib_scene_is_ready(
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
    commands.entity(trigger.target()).insert(GibReady);
}

/*
mut gib_scenes: Query<
    (&mut Transform, &mut Visibility, &GlobalTransform),
    With<SphereGibScene>,
>,
*/

#[derive(Component)]
struct GibParentGone;

// listener should ONLY be on the Sphere component.
fn destroy_sphere(
    trigger: Trigger<DestroySphere>,
    mut commands: Commands,
    gib_children: Query<&GibChild>,
    children: Query<&Children>,
    mut gib_meshes: Query<(&mut Transform, &mut Visibility, &GlobalTransform), With<Mesh3d>>,
) {
    //todo: detach gibs from the sphere, make visible, attach rigid bodies to all the components
    // and apply a fake force in the area based on where the point of contact was for the collider.
    //
    // despawn sphere

    // BeingDestroyed(Timer::new(Duration::from_secs(1), TimerMode::Once)),

    let gib_child = gib_children.get(trigger.target()).unwrap();

    commands
        .entity(gib_child.0)
        .remove::<GibReady>()
        .insert(GibParentGone);

    commands.entity(trigger.target()).try_despawn();
    // iterate through all children of the gib. not all children of the gib_parent.
    // for child in children.iter_descendants(gib_parent.0) {
    //     let Ok((mut transform, mut visibility, global_transform)) = gib_meshes.get_mut(child)
    //     else {
    //         continue;
    //     };

    //     *visibility = Visibility::Visible;
    //     *transform = global_transform.compute_transform();

    //     commands.entity(child).remove::<ChildOf>().insert((
    //         RigidBody::Dynamic,
    //         CollisionLayers::new(GameLayer::Gibs, GameLayer::Default),
    //     ));
    // }
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

fn ready_gib_bodies(
    mut commands: Commands,
    gibs: Query<Entity, Added<GibReady>>,
    children: Query<&Children>,
    meshes: Query<Entity, (With<Mesh3d>, Without<RigidBody>)>,
) {
    for gib in gibs {
        info!("added gib ready");
        for child in children.iter_descendants(gib) {
            let Ok(mesh) = meshes.get(child) else {
                continue;
            };
            commands.entity(mesh).insert((
                CollisionLayers::NONE,
                RigidBody::Dynamic,
                RigidBodyDisabled,
            ));
        }
    }
}

fn update_gib_transforms(
    mut gibs: Query<(&mut Transform, &GibsOf)>,
    transforms: Query<&Transform, Without<GibsOf>>,
) {
    for (mut gib_transform, gibs_of) in &mut gibs {
        let Ok(parent_transform) = transforms.get(gibs_of.0) else {
            continue;
        };
        *gib_transform = *parent_transform;
    }
    //todo
}

fn realize_gib_explosion(
    mut commands: Commands,
    gibs: Query<Entity, Added<GibParentGone>>,
    children: Query<&Children>,
    mut meshes: Query<(Entity, &mut Transform, &mut Visibility, &GlobalTransform), With<Mesh3d>>,
) {
    for gib in gibs {
        for child in children.iter_descendants(gib) {
            let Ok((mesh, mut trns, mut visibility, glbtrns)) = meshes.get_mut(child) else {
                continue;
            };
            commands
                .entity(mesh)
                .remove::<(ChildOf, RigidBodyDisabled)>();
            *trns = glbtrns.compute_transform();
            *visibility = Visibility::Visible;
        }
    }
}
