use bevy::prelude::*;

use crate::{
    gameplay::{GameSet, level::LevelState},
    keybinds::Keybinds,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        listen_for_restart
            .in_set(GameSet::RecordInput)
            .run_if(in_state(LevelState::Playing)),
    );
}

fn listen_for_restart(
    input: Res<ButtonInput<KeyCode>>,
    keybinds: Res<Keybinds>,
    mut level_state: ResMut<NextState<LevelState>>,
) {
    if !input.just_pressed(keybinds.restart) {
        return;
    }
    level_state.set(LevelState::NextLevel);
}
