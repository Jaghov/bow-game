use std::time::Duration;

use crate::{
    Screen,
    gameplay::{GameSet, sphere::SphereAssets},
    third_party::avian3d::GameLayer,
};

use super::Sphere;
use avian3d::prelude::*;
use bevy::{prelude::*, scene::SceneInstanceReady};

/*

New plan:
- Once the resource has been loaded, we're going to spawn a scene instance
of the gibs. When it's ready, we take all of the mesh handles and transforms, and store
them in a resource.
When a sphere is destroyed, we clone these meshes, transforms, etc. and apply it to
the position of the despawned sphere. ez.

*/

pub(super) fn plugin(app: &mut App) {
    app.add_observer(add_destroyable_sphere)
        .add_systems(OnExit(Screen::Loading), spawn_gib_scene)
        .add_systems(
            Update,
            (
                tick_being_destroyed.in_set(GameSet::TickTimers),
                despawn_destroyed.in_set(GameSet::Update),
            ),
        )
        .add_systems(Last, yoink_gib_meshes);
}

#[derive(Event)]
pub struct DestroySphere;

#[derive(Component)]
#[relationship(relationship_target = GibChild)]
pub struct GibsOf(Entity);

#[derive(Component)]
#[relationship_target(relationship = GibsOf)]
pub struct GibChild(Entity);

fn add_destroyable_sphere(trigger: Trigger<OnAdd, Sphere>, mut commands: Commands) {
    commands.entity(trigger.target()).observe(destroy_sphere);
}

#[derive(Component)]
struct BeingDestroyed(Timer);

// listener should ONLY be on the Sphere component.
fn destroy_sphere(trigger: Trigger<DestroySphere>, mut commands: Commands) {
    commands.entity(trigger.target()).try_despawn();
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

#[derive(Component)]
struct GibRoot;

fn spawn_gib_scene(mut commands: Commands, assets: Res<SphereAssets>) {
    commands
        .spawn((
            Name::new("Gib scene"),
            Visibility::Hidden,
            SceneRoot(assets.gibs.clone()),
        ))
        .observe(init_gib_resource);
}

fn init_gib_resource(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    meshes: Query<(&Transform, &GlobalTransform)>,
) {
    info!("gib scene is ready!");

    commands.entity(trigger.target()).insert(GibRoot);
}

#[derive(Resource)]
pub struct GibMeshes {
    meshes: Vec<(Transform, Handle<Mesh>)>,
}

// need to run this here and not when scene instance is ready. The global transform will be applied in post update, so it's here where we despawn the scene..
fn yoink_gib_meshes(
    mut commands: Commands,
    scene: Single<Entity, With<GibRoot>>,
    children: Query<&Children>,
    meshes: Query<(&GlobalTransform, &Mesh3d)>,
) {
    info!("yoinking gib meshes");

    let mut mesh_props = Vec::new();
    for child in children.iter_descendants(*scene) {
        let Ok((globaltransform, mesh3d)) = meshes.get(child) else {
            continue;
        };
        mesh_props.push((globaltransform.compute_transform(), mesh3d.0.clone()));
        info!(
            "Mesh Props: \nglobal_trns: {:?}",
            globaltransform.compute_transform()
        );
    }

    commands.entity(*scene).despawn();
    commands.insert_resource(GibMeshes { meshes: mesh_props });
}
