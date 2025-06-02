use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    prelude::*,
    render::{camera::CameraOutputMode, render_resource::BlendState},
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(make_hdr_compatible);
}

fn make_hdr_compatible(
    trigger: Trigger<OnAdd, Camera>,
    mut cameras: Query<&mut Camera>,
    mut commands: Commands,
) {
    let mut camera = cameras.get_mut(trigger.target()).unwrap();
    // Let the bottom-most camera dictate the tonemapping.
    // This code assumes that this camera has HDR enabled.
    if camera.order == 0 {
        return;
    }
    if camera.hdr {
        // Needed because of https://github.com/bevyengine/bevy/issues/18902
        commands.entity(trigger.target()).insert(Tonemapping::None);
    }
    // Needed because of https://github.com/bevyengine/bevy/issues/18901
    // and https://github.com/bevyengine/bevy/issues/18903
    camera.clear_color = ClearColorConfig::Custom(Color::NONE);
    camera.output_mode = CameraOutputMode::Write {
        blend_state: Some(BlendState::ALPHA_BLENDING),
        clear_color: ClearColorConfig::None,
    };
}
