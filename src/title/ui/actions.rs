use crate::{
    Screen,
    settings::SettingsState,
    theme::{interaction::OnPress, widgets},
};
use bevy::prelude::{Val::*, *};

#[derive(Component)]
struct Actions;

pub fn spawn_actions() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::Center,
            margin: UiRect::right(Px(40.)),
            row_gap: Px(20.0),
            flex_grow: 1.,
            flex_shrink: 0.,
            ..default()
        },
        Actions,
        #[cfg(target_family = "wasm")]
        {
            children![
                widgets::button("Play", transition_to_gameplay),
                widgets::button("Credits", enter_credits_screen),
                widgets::button("Settings", enter_settings),
            ]
        },
        #[cfg(not(target_family = "wasm"))]
        {
            children![
                widgets::button("Play", transition_to_gameplay),
                widgets::button("Credits", enter_credits_screen),
                widgets::button("Settings", enter_settings),
                widgets::button("Exit", exit_app),
            ]
        },
    )
}

fn transition_to_gameplay(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Transition);
}

fn enter_credits_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Credits);
}

fn enter_settings(_: Trigger<OnPress>, mut settings: ResMut<NextState<SettingsState>>) {
    settings.set(SettingsState::View);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
