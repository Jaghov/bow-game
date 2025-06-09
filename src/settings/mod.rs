use bevy::{audio::Volume, prelude::*};

mod ui;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default, Reflect)]
#[states(scoped_entities)]
pub enum SettingsState {
    #[default]
    None,
    View,
}

pub(super) fn plugin(app: &mut App) {
    app.init_state::<SettingsState>();
    app.add_plugins(ui::plugin);
    app.init_resource::<Settings>();
}

#[allow(dead_code)]
#[derive(Resource)]
pub struct Settings {
    pub sfx: Volume,
    pub music: Volume,
    pub restart: KeyCode,

    #[cfg(feature = "dev")]
    pub debug_toggle: KeyCode,
    #[cfg(feature = "dev")]
    pub inspector_toggle: KeyCode,
}

impl Settings {
    fn dan() -> Self {
        Self {
            sfx: Volume::Linear(1.),
            music: Volume::Linear(0.),
            restart: KeyCode::KeyU,
            #[cfg(feature = "dev")]
            debug_toggle: KeyCode::KeyY,
            #[cfg(feature = "dev")]
            inspector_toggle: KeyCode::KeyH,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sfx: Volume::Linear(1.),
            music: Volume::Linear(1.),
            restart: KeyCode::KeyR,
            #[cfg(feature = "dev")]
            debug_toggle: KeyCode::KeyF,
            #[cfg(feature = "dev")]
            inspector_toggle: KeyCode::KeyG,
        }
    }
}
