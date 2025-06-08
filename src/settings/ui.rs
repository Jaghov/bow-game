use bevy::{
    audio::Volume,
    input::common_conditions::input_just_pressed,
    prelude::{Val::*, *},
};

use crate::{
    settings::{Settings, SettingsState},
    theme::widgets,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(SettingsState::View), spawn_settings_menu)
        .add_systems(
            Update,
            go_back.run_if(in_state(SettingsState::View).and(input_just_pressed(KeyCode::Escape))),
        );

    app.add_systems(
        Update,
        update_music_volume_label.run_if(in_state(SettingsState::View)),
    );
}

fn spawn_settings_menu(mut commands: Commands) {
    commands.spawn((
        widgets::ui_root("Settings Menu"),
        GlobalZIndex(2),
        StateScoped(SettingsState::View),
        children![
            widgets::header("Settings"),
            settings_grid(),
            widgets::button(
                "Back",
                |_: Trigger<Pointer<Click>>, mut settings: ResMut<NextState<SettingsState>>| {
                    settings.set(SettingsState::None);
                }
            )
        ],
    ));
}
fn go_back(mut settings: ResMut<NextState<SettingsState>>) {
    settings.set(SettingsState::None);
}

fn settings_grid() -> impl Bundle {
    (
        Name::new("Settings Grid"),
        Node {
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        children![
            (
                widgets::label("Master Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            global_volume_widget(),
        ],
    )
}
const MIN_VOLUME: f32 = 0.0;
const MAX_VOLUME: f32 = 3.0;

fn global_volume_widget() -> impl Bundle {
    (
        Name::new("Global Volume Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widgets::button_small("-", lower_music_volume),
            (
                Name::new("Current Volume"),
                Node {
                    padding: UiRect::horizontal(Px(10.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(widgets::label(""), MusicVolumeLabel)],
            ),
            widgets::button_small("+", raise_music_volume),
        ],
    )
}

fn lower_music_volume(_: Trigger<Pointer<Click>>, mut settings: ResMut<Settings>) {
    let linear = (settings.music.to_linear() - 0.1).max(MIN_VOLUME);
    settings.music = Volume::Linear(linear);
}

fn raise_music_volume(_: Trigger<Pointer<Click>>, mut settings: ResMut<Settings>) {
    let linear = (settings.music.to_linear() + 0.1).min(MAX_VOLUME);
    settings.music = Volume::Linear(linear);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct MusicVolumeLabel;

fn update_music_volume_label(
    global_volume: Res<Settings>,
    mut label: Single<&mut Text, With<MusicVolumeLabel>>,
) {
    let percent = 100.0 * global_volume.music.to_linear();
    label.0 = format!("{percent:3.0}%");
}
