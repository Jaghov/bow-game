use std::time::Duration;

use bevy::prelude::*;

use crate::gameplay::{GameSet, arrow::Arrow};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, tick_flight_time.in_set(GameSet::TickTimers))
        .add_systems(PostUpdate, (reset_flight_time, despawn_arrows).chain());
}

// how long an arrow can fly without bouncing
#[derive(Component)]
pub struct MaxFlightTime(Timer);

impl Default for MaxFlightTime {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs(10), TimerMode::Once))
    }
}

fn tick_flight_time(mut timers: Query<&mut MaxFlightTime>, time: Res<Time>) {
    for mut timer in &mut timers {
        timer.0.tick(time.delta());
    }
}

fn reset_flight_time(mut timers: Query<&mut MaxFlightTime, Changed<Arrow>>) {
    for mut timer in &mut timers {
        timer.0.reset();
    }
}
fn despawn_arrows(mut commands: Commands, timers: Query<(Entity, &MaxFlightTime)>) {
    for (entity, timer) in timers {
        if timer.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
