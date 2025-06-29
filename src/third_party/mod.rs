use bevy::prelude::*;
use bevy_gltf_animation::GltfAnimationPlugin;
use bevy_mod_outline::OutlinePlugin;
use bevy_trauma_shake::TraumaPlugin;
use bevy_tweening::TweeningPlugin;

pub mod avian3d;

/// A set plugin to handle our third party plugins.
pub fn plugin(app: &mut App) {
    app.add_plugins((
        avian3d::plugin,
        TraumaPlugin,
        GltfAnimationPlugin,
        TweeningPlugin,
        OutlinePlugin,
    ));

    #[cfg(feature = "hot")]
    app.add_plugins(bevy_simple_subsecond_system::prelude::SimpleSubsecondPlugin::default());
}
