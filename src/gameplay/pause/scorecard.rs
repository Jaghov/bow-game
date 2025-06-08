use bevy::{
    color::palettes::{
        css::GRAY,
        tailwind::{YELLOW_400, YELLOW_500, YELLOW_700},
    },
    prelude::{Val::*, *},
};

use crate::gameplay::scorecard::CourseScore;

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
            ..default()
        },
        BorderRadius::all(Px(12.)),
        BackgroundColor(GRAY.into()),
    )
    //todo
}

pub fn scorecard_rows(course_no: usize, course: &CourseScore) -> impl Bundle {
    let lead = (
        Node {
            width: Px(240.),
            ..default()
        },
        Text::new(format!("Course {}", course_no + 1)),
        TextColor(Color::BLACK),
    );

    let score = (
        Node {
            width: Px(160.),
            ..default()
        },
        course.arrows_shot_ui(),
    );

    let par = (
        Node {
            width: Px(120.),
            align_items: AlignItems::End,
            ..default()
        },
        course.par_ui(),
    );

    let standards = (TextFont::from_font_size(40.),);

    (
        Node {
            display: Display::Flex,
            ..default()
        },
        BackgroundColor(YELLOW_500.into()),
        children![
            (lead, standards.clone()),
            (score, standards.clone()),
            (par, standards)
        ],
    )
}
