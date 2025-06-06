use std::time::Duration;

use crate::{
    gameplay::{
        GameSet,
        sphere::{Absorber, SphereAssets},
    },
    loading::LoadingState,
    third_party::avian3d::GameLayer,
};

use super::Sphere;
use avian3d::prelude::*;
use bevy::{prelude::*, scene::SceneInstanceReady};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(add_destroyable_sphere)
        .init_resource::<GibMeshes>()
        .add_systems(OnEnter(LoadingState::Dependencies), spawn_gib_scene)
        .add_systems(
            Update,
            (
                tick_being_destroyed.in_set(GameSet::TickTimers),
                (despawn_destroyed, limit_gib_population).in_set(GameSet::Update),
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

#[derive(Component)]
struct Gib;
// listener should ONLY be on the Sphere component.
fn destroy_sphere(
    trigger: Trigger<DestroySphere>,
    absorber: Query<(), With<Absorber>>,
    mut commands: Commands,
    meshes: Res<GibMeshes>,
    transforms: Query<(&Transform, &MeshMaterial3d<StandardMaterial>)>,
) {
    // absorbers are the exception and will be custom despawned.
    // you would ideally attach this listener to all balls but ehh why
    if absorber.get(trigger.target()).is_ok() {
        return;
    }

    let (sphere_transform, sphere_material) = transforms.get(trigger.target()).unwrap();

    let mut meshes_to_spawn = Vec::with_capacity(meshes.meshes.len());

    for (transform, mesh_handle, collider) in meshes.meshes.iter() {
        let new_transform =
            Transform::from_translation(sphere_transform.translation + transform.translation)
                .with_rotation(transform.rotation);

        meshes_to_spawn.push((
            Name::new("Gib Piece"),
            Gib,
            new_transform,
            Mesh3d(mesh_handle.clone()),
            MeshMaterial3d(sphere_material.0.clone()),
            collider.clone(),
            RigidBody::Dynamic,
            Visibility::Visible,
            CollisionLayers::new(GameLayer::Gibs, GameLayer::Default),
            BeingDestroyed(Timer::new(Duration::from_secs(3), TimerMode::Once)),
        ))
    }

    commands.entity(trigger.target()).try_despawn();
    commands.spawn_batch(meshes_to_spawn);
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
    info!("Spawning gib scene");
    commands
        .spawn((
            Name::new("Gib scene"),
            Visibility::Hidden,
            ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
            SceneRoot(assets.gibs.clone()),
        ))
        .observe(
            |trigger: Trigger<SceneInstanceReady>, mut commands: Commands| {
                commands.entity(trigger.target()).insert(GibRoot);
            },
        );
}

#[derive(Resource, Default)]
pub struct GibMeshes {
    meshes: Vec<(Transform, Handle<Mesh>, Collider)>,
    is_ready: bool,
}
impl GibMeshes {
    pub fn is_ready(&self) -> bool {
        self.is_ready
    }
}
// this function makes sure an extreme number of gibs don't exist in the world, causing lag
fn limit_gib_population(new_gibs: Query<Entity, With<Gib>>, mut commands: Commands) {
    for gib in new_gibs.iter().skip(600) {
        commands.entity(gib).try_despawn();
    }
}

// need to run this here and not when scene instance is ready. The global transform will be applied in post update, so it's here where we despawn the scene..
fn yoink_gib_meshes(
    mut commands: Commands,
    scene: Single<Entity, With<GibRoot>>,
    children: Query<&Children>,
    mut gib_mesh_res: ResMut<GibMeshes>,
    meshes: Query<(&GlobalTransform, &Mesh3d, &Collider)>,
) {
    let mut remaining_mesh_count = 0;
    info!(
        "yoinking gib meshes. Progress: {}",
        gib_mesh_res.meshes.len()
    );

    for child in children.iter_descendants(*scene) {
        let Ok((globaltransform, mesh3d, collider)) = meshes.get(child) else {
            continue;
        };
        remaining_mesh_count += 1;

        gib_mesh_res.meshes.push((
            globaltransform.compute_transform(),
            mesh3d.0.clone(),
            collider.clone(),
        ));

        commands.entity(child).despawn();
    }

    if !gib_mesh_res.meshes.is_empty() && remaining_mesh_count == 0 {
        info!("done! mesh count: {}", gib_mesh_res.meshes.len());
        gib_mesh_res.is_ready = true;
        commands.entity(*scene).despawn();
    }
}
