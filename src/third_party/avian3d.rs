use avian3d::prelude::*;
use bevy::prelude::*;

/// avian3d rocks, but it is dense.
///
/// To view the docs for 0.16, you'll have to clone the main branch
/// and run `cargo doc --open` :/
pub fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec3::Z * -9.81));
}

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    ArrowSensor, // colliders that interact with the arrow sensor.
    Arrow,       // colliders that interact with the arrow physically
    Sphere,      // colliders that interacts with all spheres
    Walls,       // colliders that interact with walls
    Backdrop,
    Gibs,
}
