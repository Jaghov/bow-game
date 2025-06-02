use bevy::prelude::*;

use crate::Screen;

mod infinite;
mod one;
mod three;
mod two;
mod zero;

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default, Reflect)]
#[source(Screen = Screen::Gameplay)]
#[states(scoped_entities)]
pub enum LevelState {
    #[default]
    Zero,
    One,
    Two,
    Three,
    Infinite,
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Level>().add_sub_state::<LevelState>();

    app.add_plugins((
        zero::plugin,
        one::plugin,
        two::plugin,
        three::plugin,
        infinite::plugin,
    ));
    //todo
}

#[derive(Resource, Default)]
pub struct Level(usize);
