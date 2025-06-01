pub mod interaction;
pub mod palette;
pub mod widgets;

use bevy::prelude::*;
pub fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}
