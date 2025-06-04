use crate::gameplay::GameState;

use super::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, sandbox.run_if(in_state(Screen::Gameplay)));
}

#[cfg_attr(feature = "hot", bevy_simple_subsecond_system::prelude::hot)]
fn sandbox(mut commands: Commands, footer: Single<Entity, With<Footer>>, assets: Res<AssetServer>) {
    use bevy::color::palettes::tailwind::GRAY_700;

    let image_node = ImageNode::new(assets.load("images/quiver.png"));

    let quiver_image = (
        image_node,
        Node {
            height: Px(75.),
            width: Px(37.5),
            ..default()
        },
    );

    let quiver_text = (
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (Text::new("Arrows"), TextColor(GRAY_700.into())),
            (
                Text::new("5"),
                TextColor(Color::BLACK),
                TextFont::from_font_size(40.),
            )
        ],
    );

    let quiver_node = (
        Node {
            padding: UiRect::axes(Px(24.), Px(12.)),
            border: UiRect::all(Px(3.)),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            column_gap: Px(15.),
            ..default()
        },
        BackgroundColor(LinearRgba::new(0.253, 0.619, 0.810, 0.5).into()),
        //BorderColor(Color::BLACK),
        BorderRadius::all(Px(12.)),
        children![quiver_image, quiver_text],
    );

    /* boxes */
    let item_image = (
        Node {
            width: Px(16.),
            height: Px(16.),
            ..default()
        },
        BackgroundColor(GRAY_700.into()),
    );

    let item_count = (Text::new("x0"), TextColor(Color::BLACK));

    let item = || {
        (
            Node {
                padding: UiRect::all(Px(8.)),
                border: UiRect::all(Px(3.)),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                column_gap: Px(2.),
                ..default()
            },
            //BorderColor(Color::BLACK),
            BackgroundColor(LinearRgba::new(1., 1., 1., 0.6).into()),
            BorderRadius::all(Px(8.)),
            children![item_image.clone(), item_count.clone()],
        )
    };

    let inventory = (
        Node {
            column_gap: Px(8.),
            align_items: AlignItems::FlexEnd,
            ..default()
        },
        children![item(), item(), item(), item(), item(), item(), item()],
    );
    //

    commands
        .entity(*footer)
        .despawn_related::<Children>()
        .insert((
            Node {
                justify_content: JustifyContent::SpaceBetween,
                margin: UiRect::all(Px(10.)),
                ..default()
            },
            children![
                (
                    Node {
                        flex_grow: 1.5,
                        ..default()
                    },
                    children![quiver_node]
                ),
                (
                    Node {
                        flex_grow: 2.,
                        ..default()
                    },
                    children![inventory]
                )
            ],
        ));
    //todod
}
