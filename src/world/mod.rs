//! This is set up for the whole world

use crate::gameplay::GAMEPLAY_CAMERA_OFFSET;
use bevy::prelude::*;

pub mod backdrop;
pub mod light;

/// This is the z-plane that everything should sit on
pub const GAME_PLANE: f32 = 0.;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((backdrop::plugin, light::plugin));
}
