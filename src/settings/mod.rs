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
    // CHANGE THIS
    app.insert_resource(Settings::dan());
}

#[allow(dead_code)]
#[derive(Resource)]
pub struct Settings {
    pub sfx: Volume,
    pub music: Volume,
    pub tutorials_enabled: bool,
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
            tutorials_enabled: true,
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
            tutorials_enabled: true,
            restart: KeyCode::KeyR,
            #[cfg(feature = "dev")]
            debug_toggle: KeyCode::KeyF,
            #[cfg(feature = "dev")]
            inspector_toggle: KeyCode::KeyG,
        }
    }
}
