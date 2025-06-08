use std::time::Duration;

use bevy::color::palettes::tailwind::GRAY_700;

use crate::{
    gameplay::{level::Level, sphere::Sphere},
    keybinds::Keybinds,
};

use super::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<BallCountState>().add_systems(
        Update,
        (
            update_level_info,
            update_restart_text,
            (tick_bctimer, update_ball_count).chain(),
        ),
    );
}

#[derive(Resource)]
struct BallCountState {
    timer: Timer,
    count: i32,
    abs_diff: i32,
}
impl Default for BallCountState {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(500), TimerMode::Once),
            count: 0,
            abs_diff: 0,
        }
    }
}

fn tick_bctimer(mut state: ResMut<BallCountState>, time: Res<Time>) {
    state.timer.tick(time.delta());
}

fn update_ball_count(
    balls: Query<(), With<Sphere>>,
    mut ball_count: Single<&mut Text, With<BallCountText>>,
    mut state: ResMut<BallCountState>,
) {
    let count = balls.iter().count() as i32;

    if state.count == count {
        return;
    }

    if state.timer.finished() {
        if count > state.count {
            state.count += 1;
        } else if count < state.count {
            state.count -= 1;
        }

        ball_count.0 = state.count.to_string();
    }

    let diff = (state.count - count).abs();
    if state.abs_diff != diff {
        state.abs_diff = diff;
        let duration = 250. / (diff as f64);
        let duration = Duration::from_millis(duration as u64);
        state.timer = Timer::new(duration, TimerMode::Once);
    }
}

fn update_restart_text(mut restart: Single<&mut Text, With<RestartText>>, keybinds: Res<Keybinds>) {
    restart.0 = "TODO".to_string();
}

#[derive(Component)]
pub struct Header;

pub fn header() -> impl Bundle {
    (
        Node {
            justify_content: JustifyContent::SpaceBetween,
            margin: UiRect::all(Px(10.)),
            ..default()
        },
        children![
            (
                Node {
                    display: Display::Flex,
                    flex_grow: 1.,
                    ..default()
                },
                children![level_info()]
            ),
            (
                Node {
                    display: Display::Flex,
                    flex_grow: 1.,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![ball_count()]
            ),
            (
                Node {
                    display: Display::Flex,
                    flex_grow: 1.,
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
                children![restart_text()]
            )
        ],
    )
}

#[derive(Component)]
pub struct LevelInfo;

fn update_level_info(level: Res<Level>, mut level_info: Single<&mut Text, With<LevelInfo>>) {
    level_info.0 = level.0.to_string();
}

fn level_info() -> impl Bundle {
    let level_info_text = (
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (Text::new("Level"), TextColor(GRAY_700.into())),
            (
                LevelInfo,
                Text::new("N/A"),
                TextColor(Color::BLACK),
                TextFont::from_font_size(30.),
            )
        ],
    );
    (
        Node {
            padding: UiRect::axes(Px(8.), Px(6.)),
            border: UiRect::all(Px(3.)),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            position_type: PositionType::Absolute,
            column_gap: Px(15.),
            ..default()
        },
        BackgroundColor(LinearRgba::new(0.253, 0.619, 0.253, 0.7).into()),
        BoxShadow::new(
            Color::srgba(0., 0., 0., 0.08),
            Px(0.),
            Px(2.),
            Px(4.),
            Px(4.),
        ),
        BorderRadius::all(Px(6.)),
        children![level_info_text],
    )
}

#[derive(Component)]
pub struct BallCountText;

fn ball_count() -> impl Bundle {
    let ball_count_text = (
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (Text::new("Targets Remaining"), TextColor(GRAY_700.into())),
            (
                BallCountText,
                Text::new("5"),
                TextColor(Color::BLACK),
                TextFont::from_font_size(40.),
            )
        ],
    );
    (
        Node {
            padding: UiRect::axes(Px(16.), Px(12.)),
            border: UiRect::all(Px(3.)),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            column_gap: Px(15.),
            ..default()
        },
        BackgroundColor(LinearRgba::new(0.253, 0.619, 0.810, 0.7).into()),
        BoxShadow::new(
            Color::srgba(0., 0., 0., 0.08),
            Px(0.),
            Px(2.),
            Px(4.),
            Px(4.),
        ),
        BorderRadius::all(Px(12.)),
        children![ball_count_text],
    )
}

#[derive(Component)]
pub struct RestartText;

fn restart_text() -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            ..default()
        },
        RestartText,
        Text::new("Press R to restart"),
        TextFont::from_font_size(30.),
    )
}
