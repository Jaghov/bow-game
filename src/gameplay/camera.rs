use bevy::{prelude::*, render::view::RenderLayers};

use crate::camera::{CameraOrder, RenderLayer};

use super::CAMERA_OFFSET;
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
            clear_color: ClearColorConfig::Custom(Color::srgba(0.05, 0.05, 0.05, 1.)),
            ..default()
        },
        Transform::from_xyz(0., 0., CAMERA_OFFSET).looking_at(Vec3::ZERO, Vec3::Y),
        MeshPickingCamera,
        Projection::from(PerspectiveProjection {
            fov: 45.0_f32.to_radians(),
            ..default()
        }),
        RenderLayers::from(RenderLayer::DEFAULT | RenderLayer::PARTICLES | RenderLayer::GIZMO3),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::CLEAR_SUNRISE,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0., 0., CAMERA_OFFSET + 5.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // commands.spawn((
    //     PointLight {
    //         shadows_enabled: true,
    //         intensity: 10_000_000.,
    //         range: 100.0,
    //         shadow_depth_bias: 0.2,
    //         ..default()
    //     },
    //     Transform::from_xyz(0.0, 16.0, 0.0),
    // ));
}
