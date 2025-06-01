use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::theme::widgets;

use super::Screen;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Win), spawn_win)
        .add_systems(
            Update,
            proceed_to_credits.run_if(in_state(Screen::Win).and(input_just_pressed(KeyCode::KeyQ))),
        );
}

fn spawn_win(mut commands: Commands) {
    commands.spawn((
        widgets::ui_root("Win Screen"),
        StateScoped(Screen::Win),
        children![
            widgets::header("You win!"),
            widgets::label("You actually *did* have time :)"),
            widgets::label("Press Q to continue")
        ],
    ));
}
fn proceed_to_credits(mut state: ResMut<NextState<Screen>>) {
    state.set(Screen::Credits);
}
