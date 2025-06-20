//! The transition from screen to title

use std::time::Duration;

use bevy::prelude::*;

use crate::{Screen, gameplay::arrow::Arrow, world::light::SetLightPosition};

const TRANSITION_DURATION: Duration = Duration::from_millis(2500);

pub mod camera;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(camera::plugin);
    app.add_systems(
        OnEnter(Screen::Transition),
        (start_transition_timer, move_light, remove_lasting_arrows),
    )
    .add_systems(OnExit(Screen::Transition), remove_duration_timer)
    .add_systems(
        PreUpdate,
        tick_duration_timer.run_if(in_state(Screen::Transition)),
    )
    .add_systems(
        PostUpdate,
        start_gameplay.run_if(in_state(Screen::Transition)),
    );
}

fn remove_lasting_arrows(mut commands: Commands, arrows: Query<Entity, With<Arrow>>) {
    for arrow in arrows {
        commands.entity(arrow).try_despawn();
    }
}

#[derive(Resource)]
struct TransitionTimer(Timer);
impl Default for TransitionTimer {
    fn default() -> Self {
        Self(Timer::new(TRANSITION_DURATION, TimerMode::Once))
    }
}
fn start_transition_timer(mut commands: Commands) {
    commands.init_resource::<TransitionTimer>();
}
fn remove_duration_timer(mut commands: Commands) {
    commands.remove_resource::<TransitionTimer>();
}
fn tick_duration_timer(mut timer: ResMut<TransitionTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());
}
fn start_gameplay(timer: Res<TransitionTimer>, mut screen: ResMut<NextState<Screen>>) {
    if !timer.0.finished() {
        return;
    }
    screen.set(Screen::Gameplay);
}

fn move_light(mut commands: Commands) {
    commands.trigger(SetLightPosition::to_gameplay().with_duration(Duration::from_millis(700)));
}
