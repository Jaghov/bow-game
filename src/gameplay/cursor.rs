use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    camera::WorldCamera,
    gameplay::{GameSet, arrow::NockedOn},
    world::GAME_PLANE,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CursorPosition>()
        .init_resource::<CursorPosition>();
    app.add_systems(Update, set_cursor_position.in_set(GameSet::RecordInput));
    app.add_systems(
        Update,
        show_cursor_if_readying_arrow.in_set(GameSet::Update),
    );
}

#[derive(Component)]
struct NockCursor;

fn show_cursor_if_readying_arrow(
    mut commands: Commands,
    arrows: Query<(), With<NockedOn>>,
    pointer: Res<CursorPosition>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cursor: Query<(Entity, &mut Transform), With<NockCursor>>,
) {
    if arrows.is_empty() {
        for (cursor, _) in cursor {
            commands.entity(cursor).despawn();
        }
        return;
    }

    let Some(point) = pointer.current() else {
        return;
    };

    if cursor.is_empty() {
        commands.spawn((
            NockCursor,
            Transform::from_translation(point).with_rotation(Quat::from_rotation_y(PI)),
            Mesh3d(meshes.add(Extrusion::new(Annulus::new(0.2, 0.3), 0.2))),
            MeshMaterial3d(materials.add(Color::WHITE)),
        ));
        return;
    }

    for (_, mut transform) in &mut cursor {
        transform.translation = point;
    }
}

/// Tells us where the pointer would be on the game plane
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct CursorPosition {
    /// this is the current current position
    /// returns Y, Z
    current: Option<Vec2>,
    /// this is the value of the last known cursor position.
    /// returns Y, Z
    last: Option<Vec2>,
}
#[allow(dead_code)]
impl CursorPosition {
    pub fn xy(&self) -> Option<Vec2> {
        self.current
    }
    pub fn current(&self) -> Option<Vec3> {
        self.current.map(|xy| Vec3::new(xy.x, xy.y, GAME_PLANE))
    }
    pub fn xy_last(&self) -> Option<Vec2> {
        self.last
    }
    pub fn last(&self) -> Option<Vec3> {
        self.last.map(|xy| Vec3::new(xy.x, xy.y, GAME_PLANE))
    }
}

fn set_cursor_position(
    camera: Query<(&Camera, &GlobalTransform), With<WorldCamera>>,
    windows: Query<&Window>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let Ok((camera, camera_transform)) = camera.single() else {
        return;
    };

    let Ok(window) = windows.single() else {
        return;
    };

    let Some(window_cursor_position) = window.cursor_position() else {
        // can happen if cursor ain't around rn
        cursor_position.current = None;
        return;
    };

    let Ok(ray) = camera.viewport_to_world(camera_transform, window_cursor_position) else {
        warn!("can't make camear ray");
        cursor_position.current = None;
        return;
    };

    let Some(distance) =
        ray.intersect_plane(Vec3::new(0., 0., GAME_PLANE), InfinitePlane3d::new(Dir3::Z))
    else {
        warn!("can't determine distance");
        cursor_position.current = None;
        return;
    };

    let point = ray.get_point(distance);

    let point2d = Vec2::new(point.x, point.y);

    cursor_position.current = Some(point2d);
    cursor_position.last = Some(point2d);
}
