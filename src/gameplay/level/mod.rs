use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    //todo
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default, Reflect)]
#[states(scoped_entities)]
pub enum LevelState {
    #[default]
    NewLevel,
    Playing,
    NextLevel,
}
