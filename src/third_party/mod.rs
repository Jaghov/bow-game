use bevy::prelude::*;
use bevy_gltf_animation::GltfAnimationPlugin;
use bevy_hanabi::HanabiPlugin;
use bevy_trauma_shake::TraumaPlugin;

pub mod avian3d;

/// A set plugin to handle our third party plugins.
pub fn plugin(app: &mut App) {
    app.add_plugins((
        avian3d::plugin,
        TraumaPlugin,
        GltfAnimationPlugin,
        HanabiPlugin,
    ));
}
