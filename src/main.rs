use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowMode};

mod asset_tracking;
mod camera;
mod credits;
#[cfg(feature = "dev")]
mod dev;
mod gameplay;
mod hdr_hack;
mod keybinds;
mod loading;
#[cfg(feature = "particles")]
mod particles;
mod rand;
mod splash;
mod theme;
mod third_party;
mod title;
mod transition;
mod world;

const UI_RENDER_LAYER: usize = 2;

fn main() -> AppExit {
    let mut app = App::new();

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

    app.register_type::<AppSystems>()
        .register_type::<Screen>()
        .init_state::<Screen>();

    app.configure_sets(
        Update,
        (
            AppSystems::TickTimers,
            AppSystems::ChangeUi,
            AppSystems::RecordInput,
            AppSystems::Update,
        )
            .chain(),
    );

    //other plugins
    app.add_plugins((
        third_party::plugin,
        asset_tracking::plugin,
        theme::plugin,
        keybinds::plugin,
        world::plugin,
        splash::plugin,
        loading::plugin,
        transition::plugin,
        title::plugin,
        gameplay::plugin,
        credits::plugin,
        camera::plugin,
        hdr_hack::plugin,
    ));

    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);

    #[cfg(feature = "particles")]
    app.add_plugins(particles::plugin);

    app.run()
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default, Reflect)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Credits,
    Transition,
    Gameplay,
}

/// High level groups of systems in the "Update" schedule.
///
/// Following the justifications of foxtrot, thought it would be nice to have now rather than later
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Reflect)]
enum AppSystems {
    /// Tick timers
    TickTimers,
    /// Update UI stuff before doing anything with input
    ChangeUi,
    /// Record player input
    RecordInput,
    /// do everything else
    Update,
}
