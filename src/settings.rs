use bevy::{audio::Volume, prelude::*};

pub(super) fn plugin(app: &mut App) {
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
            music: Volume::Linear(1.),
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
