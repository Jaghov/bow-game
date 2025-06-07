use std::path::Path;

use crate::{Screen, asset_tracking::LoadResource, theme::interaction::OnPress};
use bevy::prelude::{Val::*, *};

mod actions;
use actions::spawn_actions;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<UiAssets>()
        .load_resource::<UiAssets>()
        .add_systems(OnEnter(Screen::Title), spawn_title_screen);
}

#[cfg_attr(feature = "hot", bevy_simple_subsecond_system::prelude::hot)]
fn spawn_title_screen(mut commands: Commands) {
    {
        commands
            .spawn((
                Name::new("Title Screen"),
                Node {
                    position_type: PositionType::Absolute,
                    width: Percent(100.0),
                    height: Percent(100.0),
                    ..default()
                },
                Pickable::IGNORE,
                StateScoped(Screen::Title),
                children![(section(), Left), (spawn_actions())],
            ))
            .observe(play_click_sound_on_button_click);
    }
}
#[derive(Component)]
struct Left;

fn section() -> impl Bundle {
    (Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexEnd,
        justify_content: JustifyContent::Center,
        margin: UiRect::right(Px(40.)),
        row_gap: Px(20.0),
        flex_grow: 1.,
        flex_shrink: 0.,
        ..default()
    })
}

#[derive(Asset, Resource, Clone, Reflect)]
struct UiAssets {
    #[dependency]
    click_sfx: Handle<AudioSource>,
}

impl FromWorld for UiAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            click_sfx: asset_server.load(Path::new("audio/sfx/MenuClickSFX_V2.flac")),
        }
    }
}

fn play_click_sound_on_button_click(
    _: Trigger<OnPress>,
    mut commands: Commands,
    click: Res<UiAssets>,
) {
    commands.spawn(AudioPlayer(click.click_sfx.clone()));
}
