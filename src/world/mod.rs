//! This is set up for the whole world

use crate::gameplay::GAMEPLAY_CAMERA_OFFSET;
use bevy::prelude::*;

pub mod backdrop;

/// This is the z-plane that everything should sit on
pub const GAME_PLANE: f32 = 0.;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(backdrop::plugin);

    app.add_systems(Startup, setup_indepedent_world_entities);
}

/// components that don't need to wait on loaded assets
fn setup_indepedent_world_entities(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0., 50., GAMEPLAY_CAMERA_OFFSET + 5.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
