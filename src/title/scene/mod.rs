use bevy::prelude::*;

use crate::world::{BACKDROP_OFFSET, BLOCK_LEN};

mod camera;
mod items;

// this is the title screen position
const CAMERA_POSITION: Vec3 = Vec3::new(
    BLOCK_LEN * 7. + 5.5,
    BLOCK_LEN * 3. + 4.,
    2. - BACKDROP_OFFSET,
);

const CAMERA_LOOK_AT: Vec3 = Vec3::new(
    BLOCK_LEN * 6. + 2.5,
    BLOCK_LEN * 4. + 3.,
    -2. - BACKDROP_OFFSET,
);

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((camera::plugin, items::plugin));
}
