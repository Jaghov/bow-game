use bevy::prelude::{Val::*, *};

use crate::{
    Screen,
    gameplay::GameState,
    settings::SettingsState,
    theme::{interaction::OnPress, widgets},
};

#[derive(Component)]
struct Actions;

pub fn actions() -> impl Bundle {
    (
        Node {
            row_gap: Px(10.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        Pickable::IGNORE,
        Actions,
        children![
            widgets::header("Paused"),
            widgets::button("Settings", enter_settings),
            widgets::button("Resume", resume_game),
            widgets::button("Exit", to_title),
        ],
    )
}

fn enter_settings(_: Trigger<OnPress>, mut settings: ResMut<NextState<SettingsState>>) {
    settings.set(SettingsState::View);
}

fn resume_game(_: Trigger<OnPress>, mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Playing);
}

fn to_title(_: Trigger<OnPress>, mut game_state: ResMut<NextState<Screen>>) {
    game_state.set(Screen::Title);
}
