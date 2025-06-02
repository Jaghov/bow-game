use bevy::{
    color::palettes::tailwind::SKY_300,
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    prelude::*,
    render::view::RenderLayers,
};

use crate::{
    Screen,
    camera::{CameraOrder, RenderLayer},
    gameplay::GAMEPLAY_CAMERA_OFFSET,
    world::backdrop::{BACKDROP_OFFSET, BLOCK_LEN},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_world_camera)
        .add_systems(Update, set_camera_position.run_if(in_state(Screen::Title)));
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

#[cfg_attr(feature = "hot", bevy_simple_subsecond_system::prelude::hot)]
fn set_camera_position(mut cam: Query<&mut Transform, With<WorldCamera>>) {
    use crate::world::backdrop::BLOCK_LEN;

    let mut cam = cam.single_mut().unwrap();

    let trans_x = 3.;

    let trans_y = 3.;

    let trans_z = -1.;

    let pos = Vec3::new(
        1. + BLOCK_LEN * 7. + trans_x,
        1. + BLOCK_LEN * 3. + trans_y,
        2. + trans_z - BACKDROP_OFFSET,
    );
    let look_at = Vec3::new(
        BLOCK_LEN * 6. + trans_x,
        BLOCK_LEN * 4. + trans_y,
        -2. + trans_z - BACKDROP_OFFSET,
    );

    *cam = Transform::from_translation(pos).looking_at(look_at, Vec3::Z);
}
