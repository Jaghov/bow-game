use avian3d::prelude::*;
use bevy::prelude::*;

use crate::gameplay::{GameState, sphere::BeginDespawning};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(freeze_time)
        .add_systems(OnEnter(GameState::TimeFreeze), on_freeze)
        .add_systems(OnExit(GameState::TimeFreeze), on_unfreeze);
    //todo
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
struct FreezeParent(Entity);

pub fn freeze_time(
    trigger: Trigger<FreezeTime>,
    mut commands: Commands,
    mut state: ResMut<NextState<GameState>>,
) {
    state.set(GameState::TimeFreeze);
    let event = trigger.event();
    commands.insert_resource(FreezeParent(event.sphere));
}

fn on_freeze(mut time: ResMut<Time<Physics>>) {
    time.pause();
}

fn on_unfreeze(mut commands: Commands, freeze: Res<FreezeParent>, mut time: ResMut<Time<Physics>>) {
    commands.entity(freeze.0).trigger(BeginDespawning);
    commands.remove_resource::<FreezeParent>();
    time.unpause();
}
