use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::{Val::*, *},
};

use crate::gameplay::{
    GameState, hide_cursor, pause::scorecard::scorecard_rows, scorecard::ScoreCard, show_cursor,
};

mod actions;
use actions::actions;

mod scorecard;
use scorecard::scorecard_box;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Paused), (spawn_pause_ui, show_cursor))
        .add_systems(
            Update,
            pause.run_if(in_state(GameState::Playing).and(input_just_pressed(KeyCode::Escape))),
        )
        .add_systems(OnExit(GameState::Paused), hide_cursor);
}

fn pause(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Paused);
}

// scorecard left, ui opts right
fn spawn_pause_ui(mut commands: Commands, scorecard: Res<ScoreCard>) {
    let root = commands
        .spawn((
            Name::new("Pause Menu"),
            StateScoped(GameState::Paused),
            Node {
                width: Percent(100.),
                height: Percent(100.),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Srgba::new(0., 0., 0., 0.8).into()),
            Pickable::IGNORE,
        ))
        .id();

    let left = commands
        .spawn((
            Node {
                flex_grow: 1.,
                ..default()
            },
            ChildOf(root),
        ))
        .id();

    let scorecard_ui = commands.spawn((scorecard_box(), ChildOf(left))).id();

    for (course, course_score) in scorecard.iter().enumerate() {
        commands.spawn((scorecard_rows(course, course_score), ChildOf(scorecard_ui)));
    }

    let right = commands
        .spawn((
            Node {
                flex_grow: 1.,
                ..default()
            },
            ChildOf(root),
        ))
        .id();

    commands.spawn((actions(), ChildOf(right)));
}
