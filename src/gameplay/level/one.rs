use bevy::prelude::*;

use crate::gameplay::{
    level::LevelState,
    sphere::{SpawnSphere, SphereType},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(LevelState::One), setup_level_one);
}

fn setup_level_one(mut commands: Commands) {
    commands.trigger(SpawnSphere::new(Vec2::new(10., 0.), SphereType::Normal));
}
