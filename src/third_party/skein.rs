use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_skein::SkeinPlugin;

/// This plugin will init a protocol to communicate with the blender extension.
///
/// This enables us to add components within the actual blend file, and then reflect that
/// in the imported glb/gltf files
pub fn plugin(app: &mut App) {
    app.register_type::<StaticBody>();
    app.add_plugins(SkeinPlugin::default());
    app.add_observer(insert_static_collider);
    // this is a TODO
    //#[cfg(feature = "dev")]
    //app.add_plugins(SkeinPlugin { handle_brp: true });
}
/// Add this component in blender to create static colliders
///
/// Inserted in blender
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct StaticBody;

fn insert_static_collider(trigger: Trigger<OnAdd, StaticBody>, mut commands: Commands) {
    let collider =
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh);
    commands
        .entity(trigger.target())
        .insert((collider, RigidBody::Static));
}
