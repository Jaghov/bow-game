//! A loading screen during which game assets are loaded.
use bevy::prelude::*;

use crate::{Screen, asset_tracking::ResourceHandles, theme::widgets};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);

    app.add_systems(
        Update,
        gameplay_ready.run_if(in_state(Screen::Loading).and(all_assets_loaded)),
    );
}

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        widgets::ui_root("Loading Screen"),
        StateScoped(Screen::Loading),
        children![widgets::label("Loading...")],
    ));
}

fn gameplay_ready(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    resource_handles.is_all_done()
}
