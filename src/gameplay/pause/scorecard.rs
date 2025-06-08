use bevy::{
    color::palettes::{
        css::{GRAY, GREEN},
        tailwind::{RED_700, YELLOW_500},
    },
    prelude::{Val::*, *},
};

use crate::gameplay::scorecard::{CourseScore, ScoreCard};

pub const AT_PAR: Color = Color::BLACK;
pub const BELOW_PAR: Color = Color::Srgba(GREEN);
pub const ABOVE_PAR: Color = Color::Srgba(RED_700);

const COURSE_W: Val = Px(240.);
const SCORE_W: Val = Px(160.);
const PAR_W: Val = Px(120.);

pub fn scorecard_box() -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            row_gap: Px(5.),
            flex_direction: FlexDirection::Column,
            align_self: AlignSelf::Start,
            margin: UiRect::all(Px(20.)),
            border: UiRect::all(Px(10.)),
            ..default()
        },
        BorderColor(GRAY.into()),
        BorderRadius::all(Px(12.)),
        BackgroundColor(GRAY.into()),
        children![header()],
    )
    //todo
}

fn header() -> impl Bundle {
    let lead = (
        Node {
            width: COURSE_W,
            ..default()
        },
        Text::new("Course"),
        TextColor(Color::BLACK),
    );

    let score = (
        Node {
            width: SCORE_W,
            ..default()
        },
        TextColor(Color::BLACK),
        Text::new("Score"),
    );

    let par = (
        Node {
            width: PAR_W,
            align_items: AlignItems::End,
            ..default()
        },
        TextColor(Color::BLACK),
        Text::new("Par"),
    );

    let standards = (TextFont::from_font_size(40.),);

    (
        row_defaults(),
        children![
            (lead, standards.clone()),
            (score, standards.clone()),
            (par, standards)
        ],
    )
}

pub fn scorecard_row(course_no: usize, course: &CourseScore) -> impl Bundle {
    let lead = (
        Node {
            width: COURSE_W,
            ..default()
        },
        Text::new(format!("{}", course_no + 1)),
        TextColor(Color::BLACK),
    );

    let score = (
        Node {
            width: SCORE_W,
            ..default()
        },
        arrows_shot(course.arrows_shot(), course.course_par()),
    );

    let par = (
        Node {
            width: PAR_W,
            align_items: AlignItems::End,
            ..default()
        },
        Text::new(course.course_par().to_string()),
        TextColor(Color::BLACK),
    );

    let standards = (TextFont::from_font_size(40.),);

    (
        row_defaults(),
        children![
            (lead, standards.clone()),
            (score, standards.clone()),
            (par, standards)
        ],
    )
}

pub fn scorecard_totals(score_card: &ScoreCard) -> impl Bundle + use<> {
    let lead = (
        Node {
            width: COURSE_W,
            ..default()
        },
        Text::new("Totals"),
        TextColor(Color::BLACK),
    );

    let mut total_par = 0;
    let mut total_arrows_shot = 0;

    for course in score_card.iter() {
        let Some(score) = course.arrows_shot() else {
            continue;
        };
        total_arrows_shot += score;
        total_par += course.course_par();
        //todo
    }

    let score = (
        Node {
            width: SCORE_W,
            ..default()
        },
        arrows_shot(Some(total_arrows_shot), total_par),
    );

    let par = (
        Node {
            width: PAR_W,
            align_items: AlignItems::End,
            ..default()
        },
        Text::default(),
    );

    let standards = (TextFont::from_font_size(40.),);

    (
        row_defaults(),
        children![
            (lead, standards.clone()),
            (score, standards.clone()),
            (par, standards)
        ],
    )
}

fn arrows_shot(arrows_shot: Option<i32>, par: i32) -> impl Bundle {
    let mut arrows_fired = Text::default();
    let mut arrows_tc = TextColor(AT_PAR);

    match arrows_shot {
        Some(arrows) => {
            let diff = arrows - par;
            let sign = if diff < 0 {
                arrows_tc.0 = BELOW_PAR;
                ""
            } else if diff > 0 {
                arrows_tc.0 = ABOVE_PAR;
                "+"
            } else {
                arrows_tc.0 = AT_PAR;
                "+"
            };
            arrows_fired.0 = format!("{}({}{})", arrows, sign, diff);
        }
        None => {
            arrows_fired.0 = "".to_string();
            arrows_tc.0 = AT_PAR;
        }
    }

    (arrows_fired, arrows_tc)
}

fn row_defaults() -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            padding: UiRect::axes(Px(8.), Px(12.)),
            ..default()
        },
        BackgroundColor(YELLOW_500.into()),
    )
}
