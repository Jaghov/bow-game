//! Toggles for the different debug UIs that our plugins provide.

use crate::camera::RenderLayer;
use crate::keybinds::Keybinds;
use crate::{AppSystems, theme::widgets};
use avian3d::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::ui::Val::*;
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DebugState>();
    app.add_plugins(FpsOverlayPlugin {
        config: FpsOverlayConfig {
            enabled: false,
            ..default()
        },
    });

    app.add_plugins((
        PhysicsDebugPlugin::default(),
        PhysicsDiagnosticsPlugin,
        PhysicsDiagnosticsUiPlugin,
    ));
    app.insert_resource(PhysicsDiagnosticsUiSettings {
        enabled: false,
        ..default()
    });
    app.insert_gizmo_config(
        PhysicsGizmos::default(),
        GizmoConfig {
            enabled: false,
            render_layers: RenderLayers::from(RenderLayer::GIZMO3),
            ..default()
        },
    );
    app.insert_gizmo_config(
        LightGizmoConfigGroup {
            draw_all: true,
            ..default()
        },
        GizmoConfig {
            enabled: false,
            render_layers: RenderLayers::from(RenderLayer::GIZMO3),
            ..default()
        },
    );

    app.add_systems(Startup, setup_debug_ui_text);
    app.add_systems(
        Update,
        update_debug_ui_text.run_if(resource_exists_and_changed::<DebugState>),
    )
    .add_systems(Update, advance_debug_state)
    .add_systems(
        Update,
        (
            toggle_fps_overlay.run_if(toggled_state(DebugState::None)),
            toggle_debug_ui.run_if(toggled_state(DebugState::Ui)),
            toggle_lighting_debug_ui.run_if(toggled_state(DebugState::Lighting)),
            toggle_physics_debug_ui.run_if(toggled_state(DebugState::Physics)),
        )
            .chain()
            .in_set(AppSystems::ChangeUi),
    );
}

fn setup_debug_ui_text(mut commands: Commands) {
    commands.spawn((
        Name::new("Debug UI"),
        Node {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(100.0),
            justify_content: JustifyContent::End,
            ..default()
        },
        Pickable::IGNORE,
        children![(widgets::label("Debug UI"), DebugUiText)],
    ));
}

#[derive(Component)]
struct DebugUiText;

fn advance_debug_state(
    button_inputs: Res<ButtonInput<KeyCode>>,
    keybinds: Res<Keybinds>,
    mut debug_state: ResMut<DebugState>,
) {
    if button_inputs.just_pressed(keybinds.debug_toggle) {
        *debug_state = debug_state.next();
    }
}

fn update_debug_ui_text(
    debug_state: Res<DebugState>,
    mut text: Single<&mut Text, With<DebugUiText>>,
) {
    text.0 = match *debug_state {
        DebugState::None => "",
        DebugState::Ui => "Ui",
        DebugState::Lighting => "Lighting",
        DebugState::Physics => "Physics",
        DebugState::Landmass => "Landmass",
    }
    .to_string();
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn toggle_physics_debug_ui(
    mut config_store: ResMut<GizmoConfigStore>,
    mut physics_diagnostics: ResMut<PhysicsDiagnosticsUiSettings>,
) {
    let config = config_store.config_mut::<PhysicsGizmos>().0;
    config.enabled = !config.enabled;
    physics_diagnostics.enabled = !physics_diagnostics.enabled;
}

fn toggle_lighting_debug_ui(mut config_store: ResMut<GizmoConfigStore>) {
    let config = config_store.config_mut::<LightGizmoConfigGroup>().0;
    config.enabled = !config.enabled;
}

fn toggle_fps_overlay(mut config: ResMut<FpsOverlayConfig>) {
    config.enabled = !config.enabled;
}

#[derive(Resource, Debug, Default, Eq, PartialEq)]
enum DebugState {
    #[default]
    None,
    Ui,
    Lighting,
    Physics,
    Landmass,
}

impl DebugState {
    fn next(&self) -> Self {
        match self {
            Self::None => Self::Ui,
            Self::Ui => Self::Lighting,
            Self::Lighting => Self::Physics,
            Self::Physics => Self::Landmass,
            Self::Landmass => Self::None,
        }
    }
}

fn toggled_state(state: DebugState) -> impl Condition<()> {
    IntoSystem::into_system(move |current_state: Res<DebugState>| {
        let was_just_changed = current_state.is_changed() && !current_state.is_added();
        let entered_state = *current_state == state;
        let exited_state = *current_state == state.next();
        was_just_changed && (entered_state || exited_state)
    })
}
