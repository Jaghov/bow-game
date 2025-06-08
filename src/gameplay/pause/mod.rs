use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::{Val::*, *},
};

use crate::gameplay::GameState;

mod actions;
use actions::actions;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Paused), spawn_pause_ui)
        .add_systems(
            Update,
            pause.run_if(in_state(GameState::Playing).and(input_just_pressed(KeyCode::Escape))),
        );
}

fn pause(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Paused);
}

// scorecard left, ui opts right
fn spawn_pause_ui(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Menu"),
        StateScoped(GameState::Paused),
        Node {
            width: Percent(100.),
            height: Percent(100.),
            flex_direction: FlexDirection::Row,
            ..default()
        },
        children![actions()],
        Pickable::IGNORE,
    ));
}
