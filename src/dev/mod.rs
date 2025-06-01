//! Development tools for the game. This plugin is only enabled in dev builds.
mod debug;
mod inspector;
mod skip;

use bevy::{dev_tools::states::log_transitions, prelude::*};

use crate::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((inspector::gadget, debug::plugin, skip::plugin));
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);
}
