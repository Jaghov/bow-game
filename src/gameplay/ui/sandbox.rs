use super::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, sandbox.run_if(in_state(Screen::Gameplay)));
}

#[cfg_attr(feature = "hot", bevy_simple_subsecond_system::prelude::hot)]
fn sandbox() {}
