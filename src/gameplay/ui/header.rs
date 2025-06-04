use bevy::color::palettes::tailwind::GRAY_700;

use crate::gameplay::{bow::Quiver, sphere::Sphere};

use super::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_restart_text.run_if(resource_changed::<Quiver>),
            update_ball_count,
        ),
    );
}

fn update_ball_count(
    balls: Query<(), With<Sphere>>,
    mut ball_count: Single<&mut Text, With<BallCountText>>,
    mut gradual_count: Local<usize>,
) {
    let count = balls.iter().count();

    if count > *gradual_count {
        *gradual_count += 1;
    } else if count < *gradual_count {
        *gradual_count -= 1;
    }

    ball_count.0 = gradual_count.to_string();
}

fn update_restart_text(quiver: Res<Quiver>, mut restart: Single<&mut Text, With<RestartText>>) {
    let text = if quiver.arrow_count().is_some_and(|count| count == 0) {
        "Press R to restart".to_string()
    } else {
        String::new()
    };

    restart.0 = text;
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
            Node {
                display: Display::Flex,
                flex_grow: 1.,
                ..default()
            },
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
                    //align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
                children![restart_text()]
            )
        ],
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
        //BorderColor(Color::BLACK),
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
