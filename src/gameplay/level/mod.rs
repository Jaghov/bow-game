use bevy::prelude::*;

use crate::Screen;

#[cfg(feature = "dev")]
mod debug;

mod infinite;
mod one;
mod three;
mod two;

/// I'm so sorry levels start at one
#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default, Reflect)]
#[source(Screen = Screen::Gameplay)]
#[states(scoped_entities)]
pub enum LevelState {
    #[cfg(feature = "dev")]
    #[default]
    Debug,
    #[cfg_attr(not(feature = "dev"), default)]
    One,
    Two,
    Three,
    Infinite,
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Level>().add_sub_state::<LevelState>();

    app.add_plugins((one::plugin, two::plugin, three::plugin, infinite::plugin));

    #[cfg(feature = "dev")]
    app.add_plugins(debug::plugin);
}

#[derive(Resource)]
#[cfg_attr(feature = "dev", derive(Default))]
pub struct Level(pub usize);

#[cfg(not(feature = "dev"))]
impl Default for Level {
    fn default() -> Self {
        Level(1)
    }
}
