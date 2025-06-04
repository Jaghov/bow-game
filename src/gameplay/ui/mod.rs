use bevy::prelude::{Val::*, *};

use crate::Screen;

mod sandbox;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(sandbox::plugin);
    app.add_systems(OnEnter(Screen::Gameplay), setup);

    //todo
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("UI Root"),
        StateScoped(Screen::Gameplay),
        UiRoot,
        Node {
            width: Percent(100.),
            height: Percent(100.),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![header(), content(), footer()],
        Pickable::IGNORE,
    ));
}

#[derive(Component)]
pub struct UiRoot;

#[derive(Component)]
pub struct Header;

#[derive(Component)]
pub struct Content;

#[derive(Component)]
pub struct Footer;

fn header() -> impl Bundle {
    (Node::default(), Pickable::IGNORE, Header)
}
fn content() -> impl Bundle {
    (
        Node {
            flex_grow: 1.,
            ..default()
        },
        Pickable::IGNORE,
        Content,
    )
}
fn footer() -> impl Bundle {
    (Node::default(), Pickable::IGNORE, Footer)
}
