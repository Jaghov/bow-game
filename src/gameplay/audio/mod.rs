use bevy::{audio::Volume, prelude::*};

use crate::{Screen, asset_tracking::LoadResource};

pub fn plugin(app: &mut App) {
    app.load_resource::<MusicTracks>()
        .add_systems(OnEnter(Screen::Title), play_menu_theme)
        .add_systems(OnEnter(Screen::Transition), play_gameplay_theme)
        .add_systems(Update, (pause, mute, volume));
}
#[derive(Asset, Resource, Reflect, Clone)]
struct MusicTracks {
    #[dependency]
    menu: Handle<AudioSource>,
    #[dependency]
    game: Handle<AudioSource>,
}

impl FromWorld for MusicTracks {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            menu: asset_server.load("audio/music/Menu Theme - V2 - BevyJam6.flac"),
            game: asset_server.load("audio/music/InGame Music - V2 - BevyJam6.flac"),
        }
    }
}
#[derive(Component)]
struct Music;

fn play_menu_theme(mut commands: Commands, tracks: Res<MusicTracks>) {
    commands.spawn((
        Music,
        AudioPlayer(tracks.menu.clone()),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            ..Default::default()
        },
    ));
}

fn play_gameplay_theme(
    mut commands: Commands,
    tracks: Res<MusicTracks>,
    player: Query<(Entity), With<Music>>,
) {
    info!("Playing gameplay theme");
    let bgm = (
        AudioPlayer(tracks.game.clone()),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            ..Default::default()
        },
    );
    match player.single() {
        Ok((entity)) => {
            commands.entity(entity).remove::<AudioSink>();
            commands.entity(entity).insert(bgm);
        }
        Err(_) => {
            commands.spawn((bgm, Music));
        }
    };
}

// Toggles between menu and bgm
fn pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    music_controller: Query<&AudioSink, With<Music>>,
) {
    let Ok(sink) = music_controller.single() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::Escape) {
        sink.toggle_playback();
    }
}

fn mute(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut music_controller: Query<&mut AudioSink, With<Music>>,
) {
    let Ok(mut sink) = music_controller.single_mut() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::KeyM) {
        sink.toggle_mute();
    }
}

fn volume(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut music_controller: Query<&mut AudioSink, With<Music>>,
) {
    let Ok(mut sink) = music_controller.single_mut() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::Equal) {
        let current_volume = sink.volume();
        sink.set_volume(current_volume + Volume::Linear(0.1));
    } else if keyboard_input.just_pressed(KeyCode::Minus) {
        let current_volume = sink.volume();
        sink.set_volume(current_volume - Volume::Linear(0.1));
    }
}
