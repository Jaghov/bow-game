#![doc = r#"
All the main screen states and transitions
"#]
use bevy::prelude::*;

mod credits;
mod loading;
mod lose;
mod spawn_level;
mod splash;
mod title;
mod win;
// the gameplay module handles the gameplay screen state
use crate::gameplay;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default, Reflect)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Credits,
    SpawnLevel,
    Gameplay,
    Win,
    Lose,
}

pub fn plugin(app: &mut App) {
    app.register_type::<Screen>();

    app.init_state::<Screen>();

    // things automatically removed on state transition
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        splash::plugin,
        loading::plugin,
        title::plugin,
        spawn_level::plugin,
        gameplay::plugin,
        win::plugin,
        lose::plugin,
        credits::plugin,
    ));
}
