use bevy::prelude::*;
use bevy_kira_audio::{
    Audio, AudioApp, AudioChannel, AudioControl, AudioPlugin, AudioSource, prelude::Volume,
};

use crate::gameplay::GameState;

// This plugin is responsible to control the game audio
pub fn plugin(app: &mut App) {
    app.add_plugins(AudioPlugin)
        .add_audio_channel::<BGM>()
        .add_audio_channel::<SFX>()
        .add_systems(Startup, (load_audio, loop_bgm).chain())
        .add_systems(OnEnter(GameState::Rewinding), rewind_sfx_on_rewind)
        .add_systems(
            OnTransition {
                exited: GameState::Rewinding,
                entered: GameState::Playing,
            },
            stop_rewind_sfx_when_done,
        );
}
#[derive(Resource)]
struct AudioState {
    bgm: Handle<AudioSource>,
    rewind: Handle<AudioSource>,
    lever: Handle<AudioSource>,
    timebank: Handle<AudioSource>,
    door: Handle<AudioSource>,
    volume: f32,
}

#[derive(Resource, Default, Clone)]
struct BGM;
#[derive(Resource, Default, Clone)]
struct SFX;

fn load_audio(mut commands: Commands, audio: Res<Audio>, assets: Res<AssetServer>) {
    let bgm_handle = assets.load("audio/music/bgm.ogg");
    let rewind_handle = assets
        .load("audio/sfx/glitched_tones_household_stereo_hifi_tape_cassette_deck_rewind_589.mp3");
    let lever_handle = assets.load("reward.wav");
    let timebank_handle = assets.load("something.ogg");
    let door_handle = assets.load("door.ogg");

    let volume = 0.5;

    commands.insert_resource(AudioState {
        bgm: bgm_handle,
        rewind: rewind_handle,
        lever: lever_handle,
        timebank: timebank_handle,
        door: door_handle,
        volume,
    });
}

fn loop_bgm(audio: Res<AudioChannel<BGM>>, audio_state: Res<AudioState>) {
    audio
        .play(audio_state.bgm.clone())
        .loop_from(22.6)
        .with_volume(Volume::Amplitude(0.2));
}

fn rewind_sfx_on_rewind(audio: Res<AudioChannel<SFX>>, audio_state: Res<AudioState>) {
    audio
        .play(audio_state.rewind.clone())
        .loop_from(0.5)
        .with_volume(Volume::Amplitude(0.8));
}

fn stop_rewind_sfx_when_done(audio: Res<AudioChannel<SFX>>) {
    audio.stop();
}
