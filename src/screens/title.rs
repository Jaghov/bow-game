//! The title screen that appears when the game starts.

use bevy::prelude::*;

use crate::{
    screens::Screen,
    theme::{interaction::OnPress, widgets},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn_title_screen);
}

fn spawn_title_screen(mut commands: Commands) {
    {
        commands.spawn((
            widgets::ui_root("Title Screen"),
            StateScoped(Screen::Title),
            #[cfg(target_family = "wasm")]
            {
                children![
                    widgets::button("Play", enter_gameplay_screen),
                    widgets::button("Credits", enter_credits_screen),
                ]
            },
            #[cfg(not(target_family = "wasm"))]
            {
                children![
                    widgets::button("Play", enter_gameplay_screen),
                    widgets::button("Credits", enter_credits_screen),
                    widgets::button("Exit", exit_app),
                ]
            },
        ));
    }
}

fn enter_gameplay_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::SpawnLevel);
}

fn enter_credits_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
