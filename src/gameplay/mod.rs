use bevy::prelude::*;

mod arrow;
mod backdrop;
pub mod bow;
pub mod camera;
pub mod cursor;
mod loading;
mod particles;
mod physics;
mod sph;
mod targets;

use crate::Screen;

/// This is the z-plane that everything should sit on
pub const GAME_PLANE: f32 = 0.;

/// camera z-offset from plane
pub const CAMERA_OFFSET: f32 = 70.;

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
pub enum GameSet {
    /// Tick timers
    TickTimers,
    /// Record player input
    RecordInput,
    /// do everything else
    Update,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Reflect)]
pub enum ArrowSet {
    ProcessInput,
    UpdateBow,
    UpdateArrow,
}

pub fn plugin(app: &mut App) {
    app.add_sub_state::<GameLoadState>()
        .add_sub_state::<GameState>()
        .register_type::<GameState>()
        .register_type::<GameSet>();

    app.configure_sets(
        Update,
        (
            GameSet::TickTimers,
            GameSet::RecordInput,
            ArrowSet::ProcessInput,
            GameSet::Update,
        )
            .chain()
            .run_if(in_state(GameState::Playing)),
    );
    app.configure_sets(
        Update,
        (
            ArrowSet::ProcessInput,
            ArrowSet::UpdateBow,
            ArrowSet::UpdateArrow,
        )
            .chain()
            .run_if(in_state(GameState::Playing)),
    );

    app.add_plugins((
        particles::plugin,
        loading::plugin,
        backdrop::plugin,
        cursor::plugin,
        bow::plugin,
        sph::plugin,
        camera::plugin,
        arrow::plugin,
        targets::plugin,
    ));
}
