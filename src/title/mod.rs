//! The title screen that appears when the game starts.

use bevy::prelude::{Val::*, *};

use crate::{
    Screen,
    theme::{interaction::OnPress, widgets},
};
mod ui;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ui::plugin);
}
