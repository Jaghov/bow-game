use bevy::prelude::*;

use crate::world::backdrop::{BACKDROP_OFFSET, BLOCK_LEN};

mod camera;
mod items;

// this is the title screen position
const CAMERA_POSITION: Vec3 = Vec3::new(
    BLOCK_LEN * 7. + 4.,
    BLOCK_LEN * 3. + 4.,
    1. - BACKDROP_OFFSET,
);

const CAMERA_LOOK_AT: Vec3 = Vec3::new(
    BLOCK_LEN * 6. + 3.,
    BLOCK_LEN * 4. + 3.,
    -3. - BACKDROP_OFFSET,
);

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((camera::plugin, items::plugin));
}
