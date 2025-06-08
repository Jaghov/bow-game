use bevy::prelude::*;

use crate::{
    gameplay::GameState,
    settings::SettingsState,
    theme::{interaction::OnPress, widgets},
};

#[derive(Component)]
struct Actions;

pub fn actions() -> impl Bundle {
    (
        Node {
            flex_grow: 1.,
            ..default()
        },
        Pickable::IGNORE,
        Actions,
        children![
            widgets::header("Paused"),
            widgets::button("Settings", enter_settings),
            widgets::button("Resume", resume_game),
        ],
    )
}

fn enter_settings(_: Trigger<OnPress>, mut settings: ResMut<NextState<SettingsState>>) {
    settings.set(SettingsState::View);
}

fn resume_game(_: Trigger<OnPress>, mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Playing);
}
