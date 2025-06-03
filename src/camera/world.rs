use bevy::{
    color::palettes::tailwind::SKY_300,
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    prelude::*,
    render::view::RenderLayers,
};
use bevy_trauma_shake::Shake;

use crate::{
    camera::{CameraOrder, RenderLayer},
    gameplay::GAMEPLAY_CAMERA_OFFSET,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_world_camera);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WorldCamera;

fn spawn_world_camera(mut commands: Commands) {
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
        Shake::default(),
        Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
        Transform::from_xyz(0., 0., GAMEPLAY_CAMERA_OFFSET).looking_at(Vec3::ZERO, Vec3::Y),
        MeshPickingCamera,
        Projection::from(PerspectiveProjection {
            fov: 45.0_f32.to_radians(),
            ..default()
        }),
        RenderLayers::from(RenderLayer::DEFAULT | RenderLayer::PARTICLES | RenderLayer::GIZMO3),
        Bloom::NATURAL,
    ));
}
