use bevy::color::palettes::{
    css::{GREEN, RED},
    tailwind::GRAY_700,
};

use crate::{
    gameplay::{level::Level, mulligan::Mulligan, scorecard::ScoreCard},
    keybinds::Keybinds,
};

use super::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_ui_playing_course_score_count)
        .add_systems(
            Update,
            (
                update_mulligan_ui
                    .run_if(resource_changed::<Mulligan>.or(resource_changed::<Level>)),
                update_mulligan_keybind.run_if(resource_changed::<Keybinds>),
                update_ui_playing_course_score_count
                    .run_if(resource_changed::<ScoreCard>.or(resource_changed::<Level>)),
            ),
        );
}

fn update_ui_playing_course_score_count(
    arrows_fired: Single<
        (&mut Text, &mut TextColor),
        (With<ArrowsFiredText>, Without<CourseParText>),
    >,
    mut course_par: Single<&mut Text, (With<CourseParText>, Without<ArrowsFiredText>)>,
    scorecard: Res<ScoreCard>,
    level: Res<Level>,
) {
    const NORM: Color = Color::BLACK;
    const BELOW_PAR: Color = Color::Srgba(GREEN);
    const ABOVE_PAR: Color = Color::Srgba(RED);

    let (mut arrows_fired, mut arrows_tc) = arrows_fired.into_inner();
    let Some(course_score) = scorecard.get(level.0) else {
        arrows_fired.0 = "???".to_string();
        arrows_tc.0 = NORM;
        course_par.0 = "???".to_string();
        return;
    };

    let par = course_score.course_par();

    course_par.0 = par.to_string();

    match course_score.arrows_shot() {
        Some(arrows) => {
            let diff = arrows - par;
            let sign = if diff < 0 {
                arrows_tc.0 = BELOW_PAR;
                "-"
            } else if diff > 0 {
                arrows_tc.0 = ABOVE_PAR;
                "+"
            } else {
                arrows_tc.0 = NORM;
                ""
            };
            arrows_fired.0 = format!("{} ({}{})", arrows, sign, diff);
        }
        None => {
            arrows_fired.0 = "???".to_string();
            arrows_tc.0 = NORM;
        }
    }
}

#[derive(Component)]
pub struct Footer;

pub fn footer(keybinds: Res<Keybinds>) -> impl Bundle {
    (
        Node {
            margin: UiRect::all(Px(10.)),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        Pickable::IGNORE,
        Footer,
        children![arrowcount_node(), mulligan(keybinds)],
    )
}

#[derive(Component)]
pub struct ArrowsFiredText;

#[derive(Component)]
pub struct CourseParText;

fn update_mulligan_ui(
    mut ui: Single<&mut Node, With<UiMulliganAvailable>>,
    mulligans: Res<Mulligan>,
    level: Res<Level>,
) {
    if mulligans.can_mulligan(level.0) {
        ui.display = Display::Flex;
    } else {
        ui.display = Display::None;
    }
}

fn update_mulligan_keybind(
    mut text: Single<&mut Text, With<UiMulliganText>>,
    keybinds: Res<Keybinds>,
) {
    let keycode = format!("{:?}", keybinds.restart).split_off(3);
    text.0 = format!("Press [{}]", keycode);
}

pub fn arrowcount_node() -> impl Bundle {
    let arrows_fired_text = (
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Node {
                    column_gap: Px(6.),
                    ..default()
                },
                children![
                    (Text::new("Arrows"), TextColor(GRAY_700.into())),
                    (Text::new("Fired"), TextColor(GRAY_700.into())),
                    TextColor(GRAY_700.into()),
                ],
            ),
            (
                ArrowsFiredText,
                Text::default(),
                TextColor(Color::BLACK),
                TextFont::from_font_size(40.),
            )
        ],
    );

    let course_par_text = (
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (Text::new("Par"), TextColor(GRAY_700.into())),
            (
                CourseParText,
                Text::new("1"),
                TextColor(Color::BLACK),
                TextFont::from_font_size(40.),
            )
        ],
    );

    (
        ui_box(),
        children![
            arrows_fired_text,
            (
                Node {
                    width: Px(2.),
                    height: Percent(100.),
                    ..default()
                },
                BackgroundColor(Srgba::new(0., 0., 0., 0.4).into()),
            ),
            course_par_text
        ],
    )
}

fn ui_box() -> impl Bundle {
    (
        Node {
            padding: UiRect::axes(Px(24.), Px(12.)),
            border: UiRect::all(Px(3.)),
            flex_direction: FlexDirection::Row,
            //align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            column_gap: Px(15.),
            ..default()
        },
        BackgroundColor(LinearRgba::new(0.253, 0.619, 0.810, 0.5).into()),
        BoxShadow::new(
            Color::srgba(0., 0., 0., 0.08),
            Px(0.),
            Px(2.),
            Px(4.),
            Px(4.),
        ),
        //BorderColor(Color::BLACK),
        BorderRadius::all(Px(12.)),
    )
}

#[derive(Component)]
pub struct UiMulliganAvailable;

#[derive(Component)]
pub struct UiMulliganText;

fn mulligan(keybinds: Res<Keybinds>) -> impl Bundle {
    let keycode = format!("{:?}", keybinds.restart).split_off(3);

    (
        ui_box(),
        UiMulliganAvailable,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                (Text::new("Mulligan"), TextColor(GRAY_700.into())),
                (
                    Node {
                        flex_grow: 1.,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    children![(
                        UiMulliganText,
                        Text::new(format!("Press [{}]", keycode)),
                        TextColor(Color::BLACK),
                        TextFont::from_font_size(20.),
                    )]
                )
            ],
        )],
    )
}
