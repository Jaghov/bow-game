use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    Screen,
    gameplay::level::{Level, LevelState},
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Mulligan>()
        .add_systems(OnEnter(LevelState::NewLevel), update_mulligans)
        .add_systems(OnEnter(Screen::Gameplay), reset_mulligans);
    //todod
}

#[derive(Resource, Default)]
pub struct Mulligan {
    mulligans_used: HashMap<usize, u8>,
}

impl Mulligan {
    pub fn can_mulligan(&mut self, level: usize) -> bool {
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
