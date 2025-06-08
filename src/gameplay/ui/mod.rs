use bevy::prelude::{Val::*, *};

use crate::{Screen, settings::Keybinds};

mod footer;
pub use footer::*;

mod header;
pub use header::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((header::plugin, footer::plugin));
    app.add_systems(OnEnter(Screen::Gameplay), setup);
}

fn setup(mut commands: Commands, settings: Res<Keybinds>) {
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
        children![header(), content(), footer(settings)],
        Pickable::IGNORE,
    ));
}

#[derive(Component)]
pub struct UiRoot;

#[derive(Component)]
pub struct Content;

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
