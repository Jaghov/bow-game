//! The title screen that appears when the game starts.

use bevy::prelude::*;

mod scene;
mod ui;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((scene::plugin, ui::plugin));
    //.add_systems(OnEnter(Screen::Title), init_ui_delay)
    //.add_systems(OnExit(Screen::Title), cleanup);
}

// #[derive(Resource)]
// struct UiDelay(Timer);

// impl Default for UiDelay {
//     fn default() -> Self {
//         Self(Timer::new(Duration::from_millis(2000), TimerMode::Once))
//     }
// }

// fn init_ui_delay(mut commands: Commands) {
//     commands.init_resource::<UiDelay>();
// }

// fn cleanup(mut commands: Commands) {
//     commands.remove_resource::<UiDelay>();
// }
