use bevy::prelude::*;

mod bow;
mod loading;

use crate::Screen;

/// This for the initial load.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(Screen = Screen::Gameplay)]
#[states(scoped_entities)]
pub enum GameLoadState {
    /// This is when initially loading in
    #[default]
    Loading,
    Loaded,
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default, Reflect)]
#[source(GameLoadState = GameLoadState::Loaded)]
#[states(scoped_entities)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
}

/// High level groups of systems in the "Update" schedule.
///
/// Following the justifications of foxtrot, thought it would be nice to have now rather than later
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Reflect)]
enum GameSet {
    /// Tick timers
    TickTimers,
    /// Record player input
    RecordInput,
    /// do everything else
    Update,
}

pub fn plugin(app: &mut App) {
    app.add_sub_state::<GameLoadState>()
        .add_sub_state::<GameState>()
        .register_type::<GameState>()
        .register_type::<GameSet>();

    app.configure_sets(
        Update,
        (GameSet::TickTimers, GameSet::RecordInput, GameSet::Update)
            .chain()
            .run_if(in_state(GameState::Playing)),
    );

    app.add_plugins((loading::plugin, bow::plugin));
}
