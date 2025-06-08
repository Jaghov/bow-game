use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    Screen,
    gameplay::{
        GameSet,
        level::{Level, LevelState},
    },
    keybinds::Keybinds,
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Mulligan>()
        .add_systems(OnEnter(LevelState::NewLevel), update_mulligans)
        .add_systems(OnEnter(Screen::Gameplay), reset_mulligans);

    app.add_systems(
        Update,
        listen_for_mulligan
            .in_set(GameSet::RecordInput)
            .run_if(in_state(LevelState::Playing)),
    );
    //todod
}

#[derive(Resource, Default)]
pub struct Mulligan {
    mulligans_used: HashMap<usize, u8>,
}

impl Mulligan {
    pub fn can_mulligan(&self, level: usize) -> bool {
        match self.mulligans_used.get(&level) {
            Some(used) => *used < 2,
            None => true,
        }
    }
}

fn reset_mulligans(mut mulligans: ResMut<Mulligan>) {
    mulligans.mulligans_used.clear();
}

fn update_mulligans(level: Res<Level>, mut mulligans: ResMut<Mulligan>) {
    let used = mulligans.mulligans_used.entry(level.0).or_default();
    *used += 1;
}

fn listen_for_mulligan(
    input: Res<ButtonInput<KeyCode>>,
    keybinds: Res<Keybinds>,
    mulligan: Res<Mulligan>,
    level: Res<Level>,
    mut level_state: ResMut<NextState<LevelState>>,
) {
    if !input.just_pressed(keybinds.restart) {
        return;
    }
    if mulligan.can_mulligan(level.0) {
        level_state.set(LevelState::NextLevel);
    }
}
