//! This is set up for the whole world

use bevy::prelude::*;

pub mod backdrop;
pub mod light;

pub const BLOCK_LEN: f32 = 6.;
pub const BACKDROP_OFFSET: f32 = 5.;

/// This is the z-plane that everything should sit on
pub const GAME_PLANE: f32 = 0.;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((backdrop::plugin, light::plugin));
}
