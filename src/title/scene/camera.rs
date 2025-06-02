use std::time::Duration;

use bevy::prelude::*;

use crate::{
    Screen,
    camera::WorldCamera,
    title::{
        TitleStopwatch,
        scene::{CAMERA_LOOK_AT, CAMERA_POSITION},
    },
    transition::camera::CameraTracking,
    world::light::SetLightPosition,
};

const TITLE_SCREEN_CAM_TRANSFORM: Transform = Transform::from_translation(CAMERA_POSITION);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), start_tracking_camera)
        .add_systems(OnExit(Screen::Title), stop_tracking_camera)
        .add_systems(
            Update,
            (set_camera_position, update_light_position).run_if(in_state(Screen::Title)),
        );
}

fn start_tracking_camera(mut commands: Commands, camera: Query<&Transform, With<WorldCamera>>) {
    let start = camera.single().unwrap();
    commands.insert_resource(CameraTracking {
        start: *start,
        end: TITLE_SCREEN_CAM_TRANSFORM.looking_at(CAMERA_LOOK_AT, Vec3::Z),
    });
}
fn stop_tracking_camera(mut commands: Commands) {
    commands.remove_resource::<CameraTracking>();
}

fn set_camera_position(
    mut camera: Query<&mut Transform, (With<WorldCamera>, Without<DirectionalLight>)>,
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

#[derive(Component)]
struct Sent;

fn update_light_position(
    mut commands: Commands,
    time: Res<TitleStopwatch>,
    sent: Query<(), With<Sent>>,
) {
    if sent.single().is_ok() {
        return;
    }
    if time.0.elapsed_secs() < 1. {
        return;
    }
    info!("triggering light position");
    commands.trigger(SetLightPosition::to_below().with_duration(Duration::from_secs(3)));

    commands.spawn((Sent, StateScoped(Screen::Title)));
}
