use bevy::prelude::*;
use bevy_trauma_shake::TraumaPlugin;

pub mod avian3d;
pub mod bevy_tnua;
pub mod enhanced_input;
pub mod skein;

/// A set plugin to handle our third party plugins.
pub fn plugin(app: &mut App) {
    app.add_plugins((
        avian3d::plugin,
        bevy_tnua::plugin,
        enhanced_input::plugin,
        skein::plugin,
        TraumaPlugin,
    ));
}
