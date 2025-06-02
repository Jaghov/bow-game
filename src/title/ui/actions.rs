use crate::{
    Screen,
    theme::{interaction::OnPress, widgets},
};
use bevy::prelude::{Val::*, *};

#[derive(Component)]
struct Actions;

pub fn spawn_actions() -> impl Bundle {
    (
        super::section(),
        Actions,
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
    )
}

fn enter_gameplay_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

fn enter_credits_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
