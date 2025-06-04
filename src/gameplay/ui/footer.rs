use bevy::color::palettes::tailwind::GRAY_700;

use crate::gameplay::bow::Quiver;

use super::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_ui_quiver_count.run_if(resource_changed::<Quiver>),
    );
}

fn update_ui_quiver_count(
    quiver: Res<Quiver>,
    mut text: Single<&mut Text, With<UiQuiverCountText>>,
) {
    let value = match quiver.arrow_count() {
        Some(count) => count.to_string(),
        None => "inf".to_string(),
    };

    text.0 = value;
}

#[derive(Component)]
pub struct Footer;

pub fn footer(assets: &AssetServer) -> impl Bundle {
    (
        Node {
            justify_content: JustifyContent::SpaceBetween,
            margin: UiRect::all(Px(10.)),
            ..default()
        },
        Pickable::IGNORE,
        Footer,
        children![
            (
                Node {
                    flex_grow: 1.5,
                    ..default()
                },
                children![quiver_node(assets)]
            ),
            (
                Node {
                    flex_grow: 2.,
                    ..default()
                },
                children![inventory()]
            )
        ],
    )
}

#[derive(Component)]
pub struct UiQuiverCountText;

pub fn quiver_node(assets: &AssetServer) -> impl Bundle {
    let quiver_image = (
        ImageNode::new(assets.load("images/quiver.png")),
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
                UiQuiverCountText,
                Text::new("5"),
                TextColor(Color::BLACK),
                TextFont::from_font_size(40.),
            )
        ],
    );

    (
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
        BoxShadow::new(
            Color::srgba(0., 0., 0., 0.08),
            Px(0.),
            Px(2.),
            Px(4.),
            Px(4.),
        ),
        //BorderColor(Color::BLACK),
        BorderRadius::all(Px(12.)),
        children![quiver_image, quiver_text],
    )
}

fn inventory() -> impl Bundle {
    /* boxes */
    let item_image = (
        Node {
            width: Px(16.),
            height: Px(16.),
            ..default()
        },
        BackgroundColor(GRAY_700.into()),
    );

    let item_count = (
        Text::new("x0"),
        TextColor(Color::BLACK),
        TextFont::from_font_size(16.),
    );
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
            BoxShadow::new(
                Color::srgba(0., 0., 0., 0.08),
                Px(0.),
                Px(2.),
                Px(4.),
                Px(4.),
            ),
            //BorderColor(Color::BLACK),
            BackgroundColor(LinearRgba::new(1., 1., 1., 0.6).into()),
            BorderRadius::all(Px(8.)),
            children![item_image.clone(), item_count.clone()],
        )
    };

    (
        Node {
            column_gap: Px(8.),
            align_items: AlignItems::FlexEnd,
            ..default()
        },
        children![item(), item(), item(), item(), item(), item(), item()],
    )
}
