use std::f32::consts::PI;

use bevy::{
    input::common_conditions::{input_just_pressed, input_just_released},
    prelude::*,
};

use crate::gameplay::{
    ArrowSet, GameSet,
    arrow::{CancelArrow, FireArrow, ReadyArrow},
    bow::{ActiveBow, BowArrow},
    cursor::CursorPosition,
};

use super::{Bow, EPS};

/// how far from the bow the player must draw bow
const MAX_RADIUS: f32 = 10.;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_pull_strength.in_set(ArrowSet::ProcessInput))
        .add_systems(Update, update_pull_rotation.in_set(ArrowSet::UpdateBow))
        .add_systems(
            Update,
            on_mouse_down
                .in_set(GameSet::RecordInput)
                .run_if(input_just_pressed(MouseButton::Left)),
        )
        .add_systems(
            Update,
            on_mouse_up
                .in_set(GameSet::RecordInput)
                .run_if(input_just_released(MouseButton::Left)),
        )
        .add_systems(
            Update,
            on_mouse_cancel
                .in_set(GameSet::RecordInput)
                .run_if(input_just_pressed(MouseButton::Right)),
        );
}

fn on_mouse_down(mut commands: Commands, bows: Query<Entity, (With<Bow>, With<ActiveBow>)>) {
    for bow in bows {
        commands.trigger(ReadyArrow::for_bow(bow));
    }
}
fn on_mouse_cancel(mut commands: Commands, bows: Query<&BowArrow>) {
    for arrow in &bows {
        commands.trigger_targets(CancelArrow, arrow.arrow());
    }
}

fn on_mouse_up(mut commands: Commands, bow_arrows: Query<&BowArrow>) {
    for arrow in &bow_arrows {
        commands.trigger_targets(FireArrow, arrow.arrow());
    }
}

fn update_pull_strength(mut bow: Query<(&mut BowArrow, &Transform)>, cursor: Res<CursorPosition>) {
    let Some(cursor_position) = cursor.last() else {
        return;
    };
    let Ok((mut strength, transform)) = bow.single_mut() else {
        return;
    };
    let pull_start = transform.translation;
    let distance = (cursor_position - pull_start).length();

    // this will get clamped if the distance is greater than the max radius
    let pull_strength = distance / MAX_RADIUS;

    strength.set_strength(pull_strength);
}

fn update_pull_rotation(
    mut bow: Query<&mut Transform, (With<Bow>, With<BowArrow>)>,
    cursor: Res<CursorPosition>,
) {
    let Ok(mut bow) = bow.single_mut() else {
        return;
    };
    let Some(current_position) = cursor.current() else {
        return;
    };
    let direction = bow.translation - current_position;
    if direction.length_squared() < EPS {
        return;
    }
    let angle = direction.y.atan2(direction.x);
    bow.rotation = Quat::from_rotation_z(angle + PI);
}
