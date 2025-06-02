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

fn set_camera_position(
    mut camera: Query<&mut Transform, With<WorldCamera>>,
    time: Res<TitleStopwatch>,
    tracking: Res<CameraTracking>,
) {
    let elapsed = time.0.elapsed();

    let mut camera_transform = camera.single_mut().unwrap();

    /*
    A logistic function which essentially takes the form of

    f(x) = (COEF * x) / ( 1+e^(-k(x-x0)) ).
     */

    // Ease-in curve that starts slow and asymptotically approaches target
    let t = elapsed.as_secs_f32();
    let k = 0.2; // Controls the curve shape - higher values make it more gradual
    let m = 0.6; // "Scrunch"
    let p = 3; //power

    let eased_progress = (m * t.powi(p)) / (m * t.powi(p) + k);

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
