use bevy::prelude::*;

use crate::{
    Screen,
    camera::WorldCamera,
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
    app.add_systems(Update, set_camera_position.run_if(in_state(Screen::Title)));
}

fn set_camera_position(mut camera: Query<&mut Transform, With<WorldCamera>>, time: Res<Time>) {
    //TODO: would love to implement ease-in-out
    // Decay rate of ln(10) => after 1 second, remaining distance is 1/10th
    let decay_rate = f32::ln(4.0);

    let mut cam = camera.single_mut().unwrap();

    let title_screen_trns = TITLE_SCREEN_CAM_TRANSFORM.looking_at(LOOK_AT, Vec3::Z);

    cam.translation.smooth_nudge(
        &title_screen_trns.translation,
        decay_rate,
        time.delta_secs(),
    );

    cam.rotation
        .smooth_nudge(&title_screen_trns.rotation, decay_rate, time.delta_secs());
    //*cam = Transform::from_translation(pos).looking_at(look_at, Vec3::Z);
}
