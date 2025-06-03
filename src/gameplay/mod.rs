use avian3d::prelude::{Physics, PhysicsTime};
use bevy::prelude::*;

pub mod arrow;
pub mod bow;
pub mod cursor;
mod level;
pub mod sphere;
pub mod timefreeze;
pub mod walls;

use crate::{Screen, camera::WorldCamera};

/// camera z-offset from the gameplay plane.
///
/// This is where the camera should *usually* be when the game is being played
pub const GAMEPLAY_CAMERA_OFFSET: f32 = 100.;

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default, Reflect)]
#[source(Screen = Screen::Gameplay)]
#[states(scoped_entities)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
    TimeFreeze,
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
        (GameSet::TickTimers, GameSet::RecordInput, GameSet::Update)
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
            .chain(),
    );

    app.add_plugins((
        bow::plugin,
        sphere::plugin,
        walls::plugin,
        level::plugin,
        arrow::plugin,
        cursor::plugin,
        timefreeze::plugin,
    ))
    .add_systems(OnEnter(Screen::Gameplay), move_camera)
    .add_systems(OnEnter(GameState::Paused), pause_physics_time)
    .add_systems(OnExit(GameState::Paused), resume_physics_time);
}

// this is a hack until I implement smooth nudge
fn move_camera(mut camera: Query<&mut Transform, With<WorldCamera>>) {
    let mut camera = camera.single_mut().unwrap();

    *camera = Transform::from_xyz(0., 0., GAMEPLAY_CAMERA_OFFSET).looking_at(Vec3::ZERO, Vec3::Y);
}

fn pause_physics_time(mut time: ResMut<Time<Physics>>) {
    time.pause();
}
fn resume_physics_time(mut time: ResMut<Time<Physics>>) {
    time.unpause();
}
