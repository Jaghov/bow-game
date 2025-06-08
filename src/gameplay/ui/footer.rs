use bevy::color::palettes::tailwind::GRAY_700;

use crate::keybinds::Keybinds;

use super::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_ui_quiver_count);
}

fn update_ui_quiver_count(mut text: Single<&mut Text, With<ArrowCountText>>) {
    // let value = match quiver.arrow_count() {
    //     Some(count) => count.to_string(),
    //     None => "inf".to_string(),
    // };

    text.0 = "TODO".to_string();
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
        children![quiver_node(), mulligan(keybinds)],
    )
}

#[derive(Component)]
pub struct ArrowCountText;

#[derive(Component)]
pub struct CourseParText;

pub fn quiver_node() -> impl Bundle {
    let quiver_text = (
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
                ArrowCountText,
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
            quiver_text,
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

fn mulligan(keybinds: Res<Keybinds>) -> impl Bundle {
    let keycode = format!("{:?}", keybinds.restart).split_off(3);

    (
        ui_box(),
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
                    UiMulliganAvailable,
                    children![(
                        Text::new(format!("Press [{}]", keycode)),
                        TextColor(Color::BLACK),
                        TextFont::from_font_size(20.),
                    )]
                )
            ],
        )],
    )
}
