use bevy::{audio::Volume, prelude::*};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (pause, mute, volume));
}

fn pause(keyboard_input: Res<ButtonInput<KeyCode>>, music_controller: Query<&AudioSink>) {
    let Ok(sink) = music_controller.single() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::Space) {
        sink.toggle_playback();
    }
}

fn mute(keyboard_input: Res<ButtonInput<KeyCode>>, mut music_controller: Query<&mut AudioSink>) {
    let Ok(mut sink) = music_controller.single_mut() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::KeyM) {
        sink.toggle_mute();
    }
}

fn volume(keyboard_input: Res<ButtonInput<KeyCode>>, mut music_controller: Query<&mut AudioSink>) {
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
