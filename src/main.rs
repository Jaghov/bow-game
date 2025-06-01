use bevy::{asset::AssetMetaCheck, prelude::*, render::view::RenderLayers, window::WindowMode};
use bitflags::bitflags;

mod asset_tracking;
mod camera;
mod credits;
mod gameplay;
mod loading;
mod splash;
mod theme;
mod third_party;
mod title;

const UI_RENDER_LAYER: usize = 2;

fn main() {
    let mut app = App::new();
    app.register_type::<AppSet>()
        .register_type::<Screen>()
        .init_state::<Screen>();

    app.configure_sets(
        Update,
        (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
    );

    app.add_plugins((
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    title: "RENAME ME".to_string(),
                    fit_canvas_to_parent: true,
                    canvas: Some("#bevy".to_owned()),
                    //resolution: WindowResolution::new(1920., 1080.),
                    // might need to adjust this for WASM
                    mode: WindowMode::Windowed,
                    // Tells wasm not to override default event handling, like F5 and Ctrl+R
                    prevent_default_event_handling: false,
                    //mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }
                .into(),
                ..default()
            }),
        MeshPickingPlugin,
    ))
    .insert_resource(MeshPickingSettings {
        require_markers: true,
        ..default()
    });

    //other plugins
    app.add_plugins((
        third_party::plugin,
        asset_tracking::plugin,
        theme::plugin,
        splash::plugin,
        loading::plugin,
        title::plugin,
        gameplay::plugin,
        credits::plugin,
        camera::plugin,
    ));
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default, Reflect)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Credits,
    Gameplay,
}

/// High level groups of systems in the "Update" schedule.
///
/// Following the justifications of foxtrot, thought it would be nice to have now rather than later
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Reflect)]
enum AppSet {
    /// Tick timers
    TickTimers,
    /// Record player input
    RecordInput,
    /// do everything else
    Update,
}
