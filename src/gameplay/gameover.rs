use bevy::prelude::{Val::*, *};

use crate::{
    Screen,
    gameplay::scorecard::{ABOVE_PAR, AT_PAR, BELOW_PAR, ScoreCard, spawn_scorecard},
    theme::widgets,
    utils,
};

#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Copy, Default, Reflect)]
#[source(Screen = Screen::Gameplay)]
#[states(scoped_entities)]
pub enum GameOverState {
    #[default]
    None,
    View,
}

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<GameOverState>();
    app.add_systems(
        OnEnter(GameOverState::View),
        (spawn_gameover_ui, utils::show_cursor),
    )
    .add_systems(
        Update,
        autoreplay.run_if(resource_exists::<AutoReplay>.and(in_state(Screen::Title))),
    );
}

// scorecard left, ui opts right
fn spawn_gameover_ui(mut commands: Commands, scorecard: Res<ScoreCard>) {
    let root = commands
        .spawn((
            Name::new("Pause Menu"),
            StateScoped(GameOverState::View),
            Node {
                width: Percent(100.),
                height: Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Px(20.),
                ..default()
            },
            GlobalZIndex(1),
            BackgroundColor(Srgba::new(0., 0., 0., 0.8).into()),
            Pickable::default(),
        ))
        .id();

    let mut total_par = 0;
    let mut total_arrows_shot = 0;
    let mut perf_score_enabled = true;
    for course in scorecard.iter() {
        let Some(score) = course.arrows_shot() else {
            perf_score_enabled = false;
            continue;
        };
        total_arrows_shot += score;
        total_par += course.course_par();
    }

    let perfect_score = scorecard.iter().count() + 1;
    let diff = total_arrows_shot - total_par;

    let statement = if perf_score_enabled && total_arrows_shot as usize <= perfect_score {
        "HOLY !@#$, a perfect score! GG."
    } else {
        match diff {
            (..-3) => "Impressive! You score was almost perfect!",
            (-3..-2) => "Now that's some aim!",
            (-2..0) => "Good eye!",
            0 => "Congrats! You met the expectation. Try again?",
            (1..4) => "Decent score! can you do better?",
            (4..) => "That's tough. Try again?",
        }
    };
    let color = if diff < 0 {
        BELOW_PAR
    } else if diff > 0 {
        ABOVE_PAR
    } else {
        AT_PAR
    };

    commands.spawn((
        Node {
            padding: UiRect::axes(Px(24.), Px(16.)),
            ..default()
        },
        BackgroundColor(Srgba::new(1., 1., 1., 0.6).into()),
        BorderRadius::all(Px(12.)),
        children![(
            Text::new(statement),
            TextColor(color),
            TextFont::from_font_size(40.)
        )],
        ChildOf(root),
    ));

    spawn_scorecard(Some(root), commands.reborrow(), &scorecard);

    commands.spawn((
        widgets::button_base(
            "Play Again",
            play_again,
            (
                Node {
                    width: Px(450.0),
                    height: Px(80.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BorderRadius::MAX,
            ),
        ),
        ChildOf(root),
    ));

    commands.spawn((
        widgets::button_base(
            "Return to Title",
            return_to_title,
            (
                Node {
                    width: Px(450.0),
                    height: Px(80.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BorderRadius::MAX,
            ),
        ),
        ChildOf(root),
    ));
}

fn return_to_title(_: Trigger<Pointer<Click>>, mut state: ResMut<NextState<Screen>>) {
    state.set(Screen::Title);
}

#[derive(Resource, Default)]
pub struct AutoReplay;

fn play_again(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut state: ResMut<NextState<Screen>>,
) {
    state.set(Screen::Title);
    commands.init_resource::<AutoReplay>();
}

fn autoreplay(mut commands: Commands, mut state: ResMut<NextState<Screen>>) {
    commands.remove_resource::<AutoReplay>();
    state.set(Screen::Transition)
}
