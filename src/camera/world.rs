use bevy::{
    color::palettes::tailwind::SKY_300,
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    prelude::*,
    render::view::RenderLayers,
};

use crate::{
    camera::{CameraOrder, RenderLayer},
    world::backdrop::{BACKDROP_OFFSET, BLOCK_LEN},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_world_camera);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WorldCamera;

fn spawn_world_camera(mut commands: Commands) {
    // this is the title screen position
    let pos = Vec3::new(
        BLOCK_LEN * 7. + 4.,
        BLOCK_LEN * 3. + 4.,
        1. - BACKDROP_OFFSET,
    );

    let look_at = Vec3::new(
        BLOCK_LEN * 6. + 3.,
        BLOCK_LEN * 4. + 3.,
        -3. - BACKDROP_OFFSET,
    );

    commands.spawn((
        Name::new("World Camera"),
        Camera3d::default(),
        WorldCamera,
        Camera {
            order: CameraOrder::World.into(),
            clear_color: ClearColorConfig::Custom(SKY_300.into()),
            hdr: true,
            ..default()
        },
        Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
        Transform::from_translation(pos).looking_at(look_at, Vec3::Z),
        MeshPickingCamera,
        Projection::from(PerspectiveProjection {
            fov: 45.0_f32.to_radians(),
            ..default()
        }),
        RenderLayers::from(RenderLayer::DEFAULT | RenderLayer::PARTICLES | RenderLayer::GIZMO3),
        Bloom::NATURAL,
    ));
}
