//! A loading screen during which game assets are loaded.
use bevy::prelude::*;

use crate::{Screen, asset_tracking::ResourceHandles, gameplay::sphere::GibMeshes, theme::widgets};

#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Copy, Default, Reflect)]
#[source(Screen = Screen::Loading)]
pub enum LoadingState {
    #[default]
    Assets,
    Dependencies,
}

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<LoadingState>();
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);

    app.add_systems(
        Update,
        update_load_state.run_if(in_state(LoadingState::Assets).and(all_assets_loaded)),
    )
    .add_systems(
        Update,
        gameplay_ready.run_if(in_state(LoadingState::Dependencies).and(all_dependencies_loaded)),
    )
    .add_systems(Update, update_load_status.run_if(in_state(Screen::Loading)));
}
#[derive(Component)]
struct LoadStatus;
fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        widgets::ui_root("Loading Screen"),
        StateScoped(Screen::Loading),
        children![
            widgets::label("Loading..."),
            (LoadStatus, widgets::label("Loading Assets..."))
        ],
    ));
}

fn update_load_status(
    mut status: Single<&mut Text, With<LoadStatus>>,
    load_state: Res<State<LoadingState>>,
) {
    let text = match *load_state.get() {
        LoadingState::Assets => "Loading Assets...",
        LoadingState::Dependencies => "Doing stuff with assets...",
    };
    status.0 = text.to_string();
}

fn update_load_state(mut next_screen: ResMut<NextState<LoadingState>>) {
    info!("updating load state to dependencies!");
    next_screen.set(LoadingState::Dependencies);
}

fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    info!("resource handles!");
    resource_handles.is_all_done()
}

fn gameplay_ready(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn all_dependencies_loaded(gib_meshes: Res<GibMeshes>) -> bool {
    gib_meshes.is_ready()
}
