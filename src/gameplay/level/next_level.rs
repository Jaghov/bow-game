use std::time::Duration;

use bevy::prelude::*;

use crate::{gameplay::level::LevelState, world::light::SetLightPosition};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(LevelState::NextLevel), set_light_position);
    //todo
}

fn set_light_position(mut commands: Commands) {
    commands.trigger(
        SetLightPosition::to_wall_load_position().with_duration(Duration::from_millis(700)),
    );
}
