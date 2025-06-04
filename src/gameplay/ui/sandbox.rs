use super::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, sandbox.run_if(in_state(Screen::Gameplay)));
}

#[cfg_attr(feature = "hot", bevy_simple_subsecond_system::prelude::hot)]
fn sandbox(mut commands: Commands, header: Single<Entity, With<Header>>) {
    use bevy::color::palettes::tailwind::GRAY_700;

    let quiver_text = (
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (Text::new("Balls Remaining"), TextColor(GRAY_700.into())),
            (
                UiQuiverCountText,
                Text::new("5"),
                TextColor(Color::BLACK),
                TextFont::from_font_size(40.),
            )
        ],
    );
    let ball_count = (
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
        children![quiver_text],
    );

    let restart_text = (
        Node {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            ..default()
        },
        Text::new("Press R to restart"),
        TextFont::from_font_size(30.),
    );

    commands
        .entity(*header)
        .despawn_related::<Children>()
        .insert((
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
                    children![ball_count]
                ),
                (
                    Node {
                        display: Display::Flex,
                        flex_grow: 1.,
                        //align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexEnd,
                        ..default()
                    },
                    children![restart_text]
                )
            ],
        ));
}
