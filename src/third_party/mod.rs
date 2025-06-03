use bevy::prelude::*;
use bevy_easings::EasingsPlugin;
use bevy_gltf_animation::GltfAnimationPlugin;
use bevy_hanabi::HanabiPlugin;
use bevy_mod_outline::OutlinePlugin;
use bevy_trauma_shake::TraumaPlugin;

pub mod avian3d;

/// A set plugin to handle our third party plugins.
pub fn plugin(app: &mut App) {
    app.add_plugins((
        avian3d::plugin,
        TraumaPlugin,
        GltfAnimationPlugin,
        HanabiPlugin,
        OutlinePlugin,
        EasingsPlugin::default(),
    ));
    #[cfg(feature = "hot")]
    app.add_plugins(bevy_simple_subsecond_system::prelude::SimpleSubsecondPlugin::default());
}
