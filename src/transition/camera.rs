use bevy::prelude::*;

use crate::{
    Screen,
    camera::WorldCamera,
    gameplay::GAMEPLAY_CAMERA_OFFSET,
    transition::{TRANSITION_DURATION, TransitionTimer},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Transition), start_tracking_camera)
        .add_systems(OnExit(Screen::Transition), stop_tracking_camera)
        .add_systems(Update, move_camera.run_if(in_state(Screen::Transition)));
    //todo
}

#[derive(Resource)]
pub struct CameraTracking {
    pub start: Transform,
    pub end: Transform,
}

fn start_tracking_camera(mut commands: Commands, camera: Query<&Transform, With<WorldCamera>>) {
    let start = camera.single().unwrap();
    commands.insert_resource(CameraTracking {
        start: *start,
        end: Transform::from_xyz(0., 0., GAMEPLAY_CAMERA_OFFSET).looking_at(Vec3::ZERO, Vec3::Y),
    });
}
fn stop_tracking_camera(mut commands: Commands) {
    commands.remove_resource::<CameraTracking>();
}
fn move_camera(
    time: Res<TransitionTimer>,
    mut camera: Query<&mut Transform, With<WorldCamera>>,
    tracking: Res<CameraTracking>,
) {
    let total_duration = TRANSITION_DURATION;
    let elapsed = time.0.elapsed();

    let mut camera_transform = camera.single_mut().unwrap();

    // Calculate progress (0.0 to 1.0)
    let progress = (elapsed.as_secs_f32() / total_duration.as_secs_f32()).clamp(0.0, 1.0);

    // Apply ease-in-out curve (smoothstep)
    let eased_progress = progress * progress * (3.0 - 2.0 * progress);

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
