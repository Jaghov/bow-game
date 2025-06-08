use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::{Val::*, *},
};

use crate::{
    gameplay::{
        GameState,
        scorecard::{ScoreCard, spawn_scorecard},
    },
    utils,
};

mod actions;
use actions::actions;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Paused),
        (spawn_pause_ui, utils::show_cursor),
    )
    .add_systems(
        Update,
        pause.run_if(in_state(GameState::Playing).and(input_just_pressed(KeyCode::Escape))),
    )
    .add_systems(OnExit(GameState::Paused), utils::hide_cursor);
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
                margin: UiRect::all(Px(20.)),
                ..default()
            },
            ChildOf(root),
        ))
        .id();

    spawn_scorecard(Some(left), commands.reborrow(), &scorecard);

    let right = commands
        .spawn((
            Node {
                flex_grow: 1.,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Px(20.)),
                ..default()
            },
            ChildOf(root),
        ))
        .id();

    commands.spawn((actions(), ChildOf(right)));
}
