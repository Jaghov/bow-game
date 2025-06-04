use bevy::prelude::{Val::*, *};

use crate::Screen;

mod sandbox;

mod footer;
pub use footer::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(sandbox::plugin);
    app.add_systems(OnEnter(Screen::Gameplay), setup);

    //todo
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
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
        children![header(), content(), footer(&assets)],
        Pickable::IGNORE,
    ));
}

#[derive(Component)]
pub struct UiRoot;

#[derive(Component)]
pub struct Header;

#[derive(Component)]
pub struct Content;

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
