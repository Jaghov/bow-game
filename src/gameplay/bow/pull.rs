use bevy::prelude::*;

use crate::gameplay::GameSet;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_pull_strength.in_set(GameSet::RecordInput));
}

#[derive(Component, Default)]
pub struct PullStrength(pub f32);

fn update_pull_strength(mut strengths: Query<&mut PullStrength>, time: Res<Time>) {
    for mut strength in &mut strengths {
        strength.0 = time.elapsed_secs() % 1.;
    }
}
