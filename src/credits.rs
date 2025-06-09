//! A credits screen that can be accessed from the title screen.

use bevy::{color::palettes::tailwind::SKY_900, ecs::spawn::SpawnIter, prelude::*, ui::Val::*};

use crate::{Screen, theme::widgets};

#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Copy, Default, Reflect)]
#[source(Screen = Screen::Title)]
#[states(scoped_entities)]
pub enum CreditsState {
    #[default]
    None,
    View,
}

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<CreditsState>();
    app.add_systems(OnEnter(CreditsState::View), spawn_credits_screen);
}

fn spawn_credits_screen(mut commands: Commands) {
    commands.spawn((
        widgets::ui_root("Credits Screen"),
        BackgroundColor(SKY_900.into()),
        StateScoped(CreditsState::View),
        GlobalZIndex(4),
        children![
            widgets::header("Created by"),
            created_by(),
            widgets::header("Assets"),
            assets(),
            widgets::header("Notable Mentions"),
            notable_mentions(),
            widgets::button("Back", enter_title_screen),
        ],
    ));
}

fn created_by() -> impl Bundle {
    grid(vec![
        [
            "Daniel Gallups (dsgallups)",
            "Level design, gameplay, invariant substates, wasm crashes",
        ],
        [
            "Joseph Aghoghovbia (radifire)",
            "Level animations, SFX implementations, gameplay",
        ],
        ["Adelaide Ellicott", "All Music, Sound Effects"],
        [
            "Shane Jones (Pluggerman)",
            "Bow animation, Bolf model, Bolf balls, Arrows, Modeling",
        ],
    ])
}

fn notable_mentions() -> impl Bundle {
    grid(vec![[
        "Jan Hohenheim",
        "Mentioned bows in the bevy #offtopic channel, debugging support",
    ]])
}

fn assets() -> impl Bundle {
    grid(vec![
        [
            "Bevy logo",
            "All rights reserved by the Bevy Foundation, permission granted for splash screen use when unmodified",
        ],
        ["Level Completion SFX", "Anas (Duck165)"],
        ["Bow Model", "CC3.0 by Zsky via poly.pizza"],
    ])
}

fn grid(content: Vec<[&'static str; 2]>) -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        Children::spawn(SpawnIter(content.into_iter().flatten().enumerate().map(
            |(i, text)| {
                (
                    widgets::label(text),
                    Node {
                        justify_self: if i % 2 == 0 {
                            JustifySelf::End
                        } else {
                            JustifySelf::Start
                        },
                        ..default()
                    },
                )
            },
        ))),
    )
}

fn enter_title_screen(
    _: Trigger<Pointer<Click>>,
    mut next_screen: ResMut<NextState<CreditsState>>,
) {
    next_screen.set(CreditsState::None);
}
