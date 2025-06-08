use std::ops::DerefMut;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{
    DefaultInspectorConfigPlugin,
    bevy_egui::{EguiContext, EguiContextPass, EguiPlugin},
    bevy_inspector::hierarchy::SelectedEntities,
};
use egui_dock::egui;

use crate::settings::Settings;

pub fn gadget(app: &mut App) {
    app.register_type::<AnimationNodeIndex>();

    app.init_resource::<InspectorActive>()
        .add_plugins((
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            DefaultInspectorConfigPlugin,
        ))
        .add_systems(Update, toggle_inspector)
        .add_systems(EguiContextPass, inspector_ui.run_if(is_inspector_active));
}

#[derive(Resource, Debug, Default, Eq, PartialEq)]
struct InspectorActive(bool);

fn is_inspector_active(inspector: Res<InspectorActive>) -> bool {
    inspector.0
}

fn toggle_inspector(
    input: Res<ButtonInput<KeyCode>>,
    settings: Res<Settings>,
    mut inspector: ResMut<InspectorActive>,
) {
    if input.just_pressed(settings.inspector_toggle) {
        inspector.0 = !inspector.0;
    }
}

fn inspector_ui(world: &mut World, mut selected_entities: Local<SelectedEntities>) {
    let Ok(mut ctx) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single_mut(world)
    else {
        return;
    };

    let mut egui_context = ctx.deref_mut().clone();
    egui::SidePanel::left("hierarchy").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            ui.heading("Hierarchy");

            bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(
                world,
                ui,
                &mut selected_entities,
            );

            ui.label("Press escape to toggle UI");
            ui.allocate_space(ui.available_size());
        });
    });

    egui::SidePanel::right("inspector")
        .default_width(250.0)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("Inspector");

                match selected_entities.as_slice() {
                    &[entity] => {
                        bevy_inspector_egui::bevy_inspector::ui_for_entity(world, entity, ui);
                    }
                    entities => {
                        bevy_inspector_egui::bevy_inspector::ui_for_entities_shared_components(
                            world, entities, ui,
                        );
                    }
                }

                ui.allocate_space(ui.available_size());
            });
        });
}

// #[derive(Component)]
// struct Rotator;

// fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
//     for mut transform in &mut query {
//         transform.rotate_x(3.0 * time.delta_secs());
//     }
// }
