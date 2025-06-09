use avian3d::prelude::*;
use bevy::prelude::*;

use crate::gameplay::{GameState, arrow::FireArrow, sphere::DestroySphere};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(freeze_time)
        .add_systems(OnEnter(GameState::TimeFreeze), on_freeze)
        .add_systems(OnExit(GameState::TimeFreeze), on_unfreeze)
        .add_observer(listen_for_fire);
}

#[derive(Event)]
pub struct FreezeTime {
    /// the sphere hit to freeze time
    sphere: Entity,
}
impl FreezeTime {
    pub fn new(sphere: Entity) -> Self {
        Self { sphere }
    }
}
#[derive(Resource)]
pub struct FreezeLocation {
    sphere: Entity,
    pub location: Vec3,
}

pub fn freeze_time(
    trigger: Trigger<FreezeTime>,
    mut commands: Commands,
    mut state: ResMut<NextState<GameState>>,
    transforms: Query<&Transform>,
) {
    let event = trigger.event();

    let Ok(transform) = transforms.get(event.sphere) else {
        return;
    };
    commands.insert_resource(FreezeLocation {
        sphere: event.sphere,
        location: transform.translation,
    });
    state.set(GameState::TimeFreeze);
}

fn on_freeze(mut time: ResMut<Time<Physics>>) {
    time.pause();
}

fn on_unfreeze(
    mut commands: Commands,
    freeze: Res<FreezeLocation>,
    mut time: ResMut<Time<Physics>>,
) {
    commands.trigger_targets(DestroySphere, freeze.sphere);
    commands.remove_resource::<FreezeLocation>();
    time.unpause();
}

fn listen_for_fire(_trigger: Trigger<FireArrow>, mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Playing);
}
