use bevy::prelude::*;

pub fn hide_cursor(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut().unwrap();

    primary_window.cursor_options.visible = false;
}

pub fn show_cursor(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut().unwrap();

    primary_window.cursor_options.visible = true;
}
