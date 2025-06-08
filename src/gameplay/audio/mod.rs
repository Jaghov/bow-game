use bevy::prelude::*;

use crate::{Screen, asset_tracking::LoadResource, settings::Settings};

pub fn plugin(app: &mut App) {
    app.load_resource::<MusicTracks>()
        .add_systems(OnEnter(Screen::Title), play_menu_theme)
        .add_systems(OnEnter(Screen::Transition), play_gameplay_theme)
        .add_systems(Update, volume.run_if(resource_changed::<Settings>));
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

fn play_menu_theme(
    mut commands: Commands,
    tracks: Res<MusicTracks>,
    players: Query<Entity, With<Music>>,
    settings: Res<Settings>,
) {
    for player in players {
        commands.entity(player).despawn();
    }
    commands.spawn((
        Music,
        AudioPlayer(tracks.menu.clone()),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: settings.music,
            ..Default::default()
        },
    ));
}

fn play_gameplay_theme(
    mut commands: Commands,
    tracks: Res<MusicTracks>,
    player: Query<(Entity), With<Music>>,
    settings: Res<Settings>,
) {
    info!("Playing gameplay theme");
    let bgm = (
        AudioPlayer(tracks.game.clone()),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: settings.music,
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

fn volume(settings: Res<Settings>, mut music_controller: Query<&mut AudioSink, With<Music>>) {
    let Ok(mut sink) = music_controller.single_mut() else {
        return;
    };

    sink.set_volume(settings.music);
}
