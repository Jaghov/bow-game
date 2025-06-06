//!plugin for the user's primary bow

use std::{collections::VecDeque, f32::consts::PI};

use bevy::{
    input::common_conditions::{input_just_pressed, input_just_released},
    prelude::*,
};

use crate::{
    Screen,
    gameplay::{
        GameSet, GameState,
        arrow::{ARROW_VELOCITY_THRESHOLD, CancelArrow, FireArrow, ReadyArrow},
        bow::{Bow, BowArrow, BowAssets, EPS, animation},
        cursor::CursorPosition,
    },
};

mod quiver;
pub use quiver::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PrimaryBow>();

    app.add_plugins(quiver::plugin);

    app.add_systems(OnEnter(Screen::Gameplay), spawn_primary_bow)
        .add_systems(
            Update,
            (
                update_primary_bow_transform,
                update_primary_bow_rotation_not_pulling,
            )
                .run_if(in_state(GameState::Playing))
                .in_set(GameSet::Update),
        )
        .add_systems(
            Update,
            (
                on_mouse_cancel.run_if(input_just_pressed(MouseButton::Right)),
                on_mouse_up.run_if(input_just_released(MouseButton::Left)),
                on_mouse_down.run_if(input_just_pressed(MouseButton::Left)),
            )
                .run_if(in_state(GameState::Playing))
                .in_set(GameSet::RecordInput),
        );
}

// this is the bow on the cursor
#[derive(Component, Reflect)]
pub struct PrimaryBow;

fn spawn_primary_bow(mut commands: Commands, assets: Res<BowAssets>) {
    info!("Spawning bow");
    commands
        .spawn((
            Name::new("Primary Bow"),
            StateScoped(Screen::Gameplay),
            Bow,
            PrimaryBow,
            Transform::default().with_scale(Vec3::splat(2.)),
            SceneRoot(assets.scene.clone()),
        ))
        .observe(animation::setup_animations);
}

fn update_primary_bow_transform(
    cursor: Res<CursorPosition>,
    mut bow: Query<&mut Transform, (With<Bow>, With<PrimaryBow>, Without<BowArrow>)>,
) {
    let Ok(mut bow) = bow.single_mut() else {
        return;
    };
    let Some(position) = cursor.current() else {
        return;
    };
    bow.translation = position;
}

fn update_primary_bow_rotation_not_pulling(
    cursor: Res<CursorPosition>,
    mut bow: Query<&mut Transform, (With<Bow>, Without<BowArrow>)>,
    mut last_positions: Local<VecDeque<Vec3>>,
    mut bow_should_rotation: Local<Quat>,
) {
    //number of positions to keep track of
    const NUM_POS_TO_TRACK: usize = 5;
    const CURSOR_POS_THRESHOLD: f32 = 3.;
    const ROTATION_SPEED: f32 = 0.15;

    let Some(position) = cursor.current() else {
        return;
    };
    /*
    if the number of positions is < 5, push regardless.
    if positions == 5, determine if
    */
    let mut adjust_should_rot = false;
    if last_positions.len() < NUM_POS_TO_TRACK {
        last_positions.push_back(position);
        adjust_should_rot = true;
    } else if last_positions.back().is_some_and(|lp| {
        (lp.x - position.x).abs() > CURSOR_POS_THRESHOLD
            || (lp.y - position.y).abs() > CURSOR_POS_THRESHOLD
    }) {
        last_positions.pop_front();
        last_positions.push_back(position);
        adjust_should_rot = true;
    }

    if adjust_should_rot && last_positions.len() >= 2 {
        let mut weighted_direction = Vec3::ZERO;
        let mut total_weight = 0.0;

        // Calculate weighted direction from consecutive position pairs
        for i in 1..last_positions.len() {
            let prev_pos = last_positions[i - 1];
            let curr_pos = last_positions[i];
            let direction = curr_pos - prev_pos;

            // Give more weight to recent movements
            let weight = i as f32;
            weighted_direction += direction * weight;
            total_weight += weight;
        }

        if total_weight > 0.0 && weighted_direction.length() > EPS {
            weighted_direction /= total_weight;
            let angle = weighted_direction.y.atan2(weighted_direction.x);
            *bow_should_rotation = Quat::from_rotation_z(angle + PI);
        }
    }

    let Ok(mut bow) = bow.single_mut() else {
        return;
    };

    // Smoothly interpolate to the target rotation
    bow.rotation = bow.rotation.slerp(*bow_should_rotation, ROTATION_SPEED);
}

fn on_mouse_down(
    mut commands: Commands,
    bow: Single<Entity, (With<PrimaryBow>)>,
    quiver: Res<Quiver>,
) {
    if quiver.can_fire() {
        commands.trigger(ReadyArrow::for_bow(*bow));
    }
}
fn on_mouse_cancel(mut commands: Commands, bows: Query<&BowArrow>) {
    for arrow in &bows {
        commands.trigger_targets(CancelArrow, arrow.arrow());
    }
}

fn on_mouse_up(mut commands: Commands, bow_arrows: Query<&BowArrow>, mut quiver: ResMut<Quiver>) {
    for arrow in &bow_arrows {
        commands.trigger_targets(FireArrow, arrow.arrow());
        if arrow.arrow_velocity() > ARROW_VELOCITY_THRESHOLD {
            quiver.remove_arrow();
        }
    }
}
