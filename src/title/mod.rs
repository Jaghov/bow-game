//! The title screen that appears when the game starts.

use bevy::{prelude::*, time::Stopwatch};

use crate::{Screen, utils};

mod scene;
mod ui;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((scene::plugin, ui::plugin))
        .add_systems(
            OnEnter(Screen::Title),
            (start_transition_timer, utils::show_cursor),
        )
        .add_systems(OnExit(Screen::Title), remove_duration_timer)
        .add_systems(
            PreUpdate,
            tick_duration_timer.run_if(in_state(Screen::Title)),
        );
}

#[derive(Resource, Default)]
struct TitleStopwatch(Stopwatch);

fn start_transition_timer(mut commands: Commands) {
    commands.init_resource::<TitleStopwatch>();
}
fn remove_duration_timer(mut commands: Commands) {
    commands.remove_resource::<TitleStopwatch>();
}

fn tick_duration_timer(mut timer: ResMut<TitleStopwatch>, time: Res<Time>) {
    timer.0.tick(time.delta());
}
