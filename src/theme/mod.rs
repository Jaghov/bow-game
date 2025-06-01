pub mod interaction;
pub mod palette;
//#[cfg(not(feature = "dev"))]
pub mod shader;
pub mod widgets;

use bevy::prelude::*;
pub fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
    //#[cfg(not(feature = "dev"))]
    app.add_plugins(shader::plugin).add_plugins(palette::plugin);
}
