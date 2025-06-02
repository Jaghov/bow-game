use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // CHANGE THIS
    app.insert_resource(Keybinds::dan());
}

#[allow(dead_code)]
#[derive(Resource)]
pub struct Keybinds {
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_rotate_left: KeyCode,
    pub key_rotate_right: KeyCode,

    #[cfg(feature = "dev")]
    pub debug_toggle: KeyCode,
    #[cfg(feature = "dev")]
    pub inspector_toggle: KeyCode,
}

impl Keybinds {
    fn dan() -> Self {
        Self {
            key_up: KeyCode::KeyI,
            key_down: KeyCode::KeyK,
            key_left: KeyCode::KeyJ,
            key_right: KeyCode::KeyL,
            key_rotate_left: KeyCode::KeyU,
            key_rotate_right: KeyCode::KeyO,
            #[cfg(feature = "dev")]
            debug_toggle: KeyCode::KeyY,
            #[cfg(feature = "dev")]
            inspector_toggle: KeyCode::KeyH,
        }
    }
}

impl Default for Keybinds {
    fn default() -> Self {
        Self {
            key_up: KeyCode::KeyW,
            key_down: KeyCode::KeyS,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_rotate_left: KeyCode::KeyQ,
            key_rotate_right: KeyCode::KeyE,
            #[cfg(feature = "dev")]
            debug_toggle: KeyCode::KeyF,
            #[cfg(feature = "dev")]
            inspector_toggle: KeyCode::KeyG,
        }
    }
}
