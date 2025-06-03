use bevy::prelude::*;

use crate::Screen;

mod infinite;
mod one;
mod three;
mod two;

/// I'm so sorry levels start at one
#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default, Reflect)]
#[source(Screen = Screen::Gameplay)]
#[states(scoped_entities)]
pub enum LevelState {
    #[default]
    One,
    Two,
    Three,
    Infinite,
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Level>().add_sub_state::<LevelState>();

    app.add_plugins((one::plugin, two::plugin, three::plugin, infinite::plugin));
    //todo
}

#[derive(Resource)]
pub struct Level(pub usize);

impl Default for Level {
    fn default() -> Self {
        Level(1)
    }
}
