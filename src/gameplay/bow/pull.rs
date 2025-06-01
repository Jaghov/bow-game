use bevy::prelude::*;

use crate::gameplay::GameSet;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_pull_strength.in_set(GameSet::RecordInput));
}

#[derive(Component, Default)]
pub struct PullStrength(f32);

impl PullStrength {
    /// Returns a value from 0. to 1.
    pub fn strength(&self) -> f32 {
        self.0
    }
    /// clamps a set value from 0 to 1
    pub fn set_strength(&mut self, val: f32) {
        self.0 = val.clamp(0., 1.)
    }
}

fn update_pull_strength(mut strengths: Query<&mut PullStrength>, time: Res<Time>) {
    for mut strength in &mut strengths {
        // repeat every two seconds
        let repeat = 2.;
        let initial_strength = time.elapsed_secs() % repeat;
        strength.set_strength(initial_strength);
    }
}
