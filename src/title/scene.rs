use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::{
    Screen,
    camera::WorldCamera,
    transition::camera::CameraTracking,
    world::backdrop::{BACKDROP_OFFSET, BLOCK_LEN},
};

// this is the title screen position
const POS: Vec3 = Vec3::new(
    BLOCK_LEN * 7. + 4.,
    BLOCK_LEN * 3. + 4.,
    1. - BACKDROP_OFFSET,
);

const LOOK_AT: Vec3 = Vec3::new(
    BLOCK_LEN * 6. + 3.,
    BLOCK_LEN * 4. + 3.,
    -3. - BACKDROP_OFFSET,
);
const TITLE_SCREEN_CAM_TRANSFORM: Transform = Transform::from_translation(POS);

const TRANSITION_DURATION: Duration = Duration::from_millis(2500);

#[derive(Resource, Default)]
struct TransitionTimer(Stopwatch);

fn start_transition_timer(mut commands: Commands) {
    commands.init_resource::<TransitionTimer>();
}
fn remove_duration_timer(mut commands: Commands) {
    commands.remove_resource::<TransitionTimer>();
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Title),
        (start_transition_timer, start_tracking_camera),
    )
    .add_systems(
        OnExit(Screen::Title),
        (remove_duration_timer, stop_tracking_camera),
    )
    .add_systems(
        PreUpdate,
        tick_duration_timer.run_if(in_state(Screen::Title)),
    )
    .add_systems(Update, set_camera_position.run_if(in_state(Screen::Title)));
}
fn start_tracking_camera(mut commands: Commands, camera: Query<&Transform, With<WorldCamera>>) {
    let start = camera.single().unwrap();
    commands.insert_resource(CameraTracking {
        start: *start,
        end: TITLE_SCREEN_CAM_TRANSFORM.looking_at(LOOK_AT, Vec3::Z),
    });
}
fn stop_tracking_camera(mut commands: Commands) {
    commands.remove_resource::<CameraTracking>();
}
fn tick_duration_timer(mut timer: ResMut<TransitionTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());
}

fn set_camera_position(
    mut camera: Query<&mut Transform, With<WorldCamera>>,
    time: Res<TransitionTimer>,
    tracking: Res<CameraTracking>,
) {
    let total_duration = TRANSITION_DURATION;
    let elapsed = time.0.elapsed();

    let mut camera_transform = camera.single_mut().unwrap();

    // Ease-in curve that starts slow and asymptotically approaches target
    let t = elapsed.as_secs_f32();
    let k = 0.8; // Controls the curve shape - higher values make it more gradual

    let eased_progress = (t * t) / (t * t + k);

    // Interpolate translation
    let new_translation = tracking
        .start
        .translation
        .lerp(tracking.end.translation, eased_progress);

    // Interpolate rotation
    let new_rotation = tracking
        .start
        .rotation
        .slerp(tracking.end.rotation, eased_progress);

    // Apply the interpolated transform
    camera_transform.translation = new_translation;
    camera_transform.rotation = new_rotation;
}
