use bevy::prelude::*;
use std::time::Duration;

use crate::gameplay::{GameSet, level::LevelState};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(LevelState::NewLevel), init_timer)
        .add_systems(
            Update,
            tick_timer
                .in_set(GameSet::TickTimers)
                .run_if(in_state(LevelState::NewLevel)),
        )
        .add_systems(
            PostUpdate,
            update_level_state.run_if(in_state(LevelState::NewLevel)),
        );
}

#[derive(Resource)]
pub struct LevelSetupTimer(Timer);

impl LevelSetupTimer {
    /// 0.0 to 1.
    pub fn fraction(&self) -> f32 {
        self.0.fraction()
    }

    pub fn wall_progress(&self) -> f32 {
        //wall will be ready at 0.7
        let wall_done_at = 0.7;
        (self.fraction() / wall_done_at).clamp(0.0, 1.0)
    }
    pub fn sphere_progress(&self) -> f32 {
        //light will start moving at 0.7
        let light_start = 0.7;

        let total_of_frac = 1. - light_start;
        let amt = (self.fraction() - light_start).clamp(0., total_of_frac);

        amt / total_of_frac
    }
}

impl Default for LevelSetupTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs(2), TimerMode::Once))
    }
}

fn init_timer(mut commands: Commands) {
    commands.init_resource::<LevelSetupTimer>();
}
fn tick_timer(mut timer: ResMut<LevelSetupTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());
}

fn update_level_state(
    mut commands: Commands,
    timer: Res<LevelSetupTimer>,
    mut level_state: ResMut<NextState<LevelState>>,
) {
    if !timer.0.finished() {
        return;
    }
    commands.remove_resource::<LevelSetupTimer>();
    level_state.set(LevelState::Playing);
}
