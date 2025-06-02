use bevy::prelude::*;

pub mod arrow;
pub mod bow;
pub mod cursor;
pub mod sph;
mod targets;

use crate::{Screen, camera::WorldCamera};

/// camera z-offset from the gameplay plane.
///
/// This is where the camera should *usually* be when the game is being played
pub const GAMEPLAY_CAMERA_OFFSET: f32 = 70.;

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default, Reflect)]
#[source(Screen = Screen::Gameplay)]
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
    app.add_sub_state::<GameState>()
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
        bow::plugin,
        sph::plugin,
        arrow::plugin,
        targets::plugin,
        cursor::plugin,
    ))
    .add_systems(OnEnter(Screen::Gameplay), move_camera);
}

// this is a hack until I implement smooth nudge
fn move_camera(mut camera: Query<&mut Transform, With<WorldCamera>>) {
    let mut camera = camera.single_mut().unwrap();

    *camera = Transform::from_xyz(0., 0., GAMEPLAY_CAMERA_OFFSET).looking_at(Vec3::ZERO, Vec3::Y);
}
