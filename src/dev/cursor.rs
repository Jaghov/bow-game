use std::f32::consts::PI;

use bevy::prelude::*;

use crate::gameplay::{GameSet, cursor::CursorPosition};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, pointer_pos.in_set(GameSet::Update));
}

fn pointer_pos(pointer: Res<CursorPosition>, mut gizmos: Gizmos) {
    let Some(point) = pointer.current() else {
        return;
    };

    // Draw a circle just above the ground plane at that position.
    gizmos.circle(
        Isometry3d::new(point, Quat::from_rotation_y(PI)),
        0.2,
        Color::WHITE,
    );
}
