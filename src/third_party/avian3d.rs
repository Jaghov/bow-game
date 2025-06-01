use avian3d::prelude::*;
use bevy::prelude::*;

/// avian3d rocks, but it is dense.
///
/// To view the docs for 0.16, you'll have to clone the main branch
/// and run `cargo doc --open` :/
pub fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default());
}

// /// This is used to determine what things should interact on collision.
// ///
// /// In our case, this will be something like the time prop.
// ///
// /// We should be able to "walk through" the prop, so no collision
// #[derive(Debug, PhysicsLayer, Default, Reflect)]
// pub enum CollisionLayer {
//     /// Things like the ground, normal objects.
//     #[default]
//     Default,
//     /// Something that can be picked up
//     Prop,
//     /// You!
//     Character,
// }
