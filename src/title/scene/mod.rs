use bevy::prelude::*;

mod camera;
mod items;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((camera::plugin, items::plugin));
}
