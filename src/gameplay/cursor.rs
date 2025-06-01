use bevy::{math::VectorSpace, prelude::*};

use crate::camera::WorldCamera;

use super::{GAME_PLANE, GameSet};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CursorPosition>()
        .init_resource::<CursorPosition>();
    app.add_systems(Update, set_cursor_position.in_set(GameSet::RecordInput));
}

/// Tells us where the pointer would be on the game plane
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct CursorPosition {
    /// returns Y, Z
    current: Option<Vec2>,
    /// returns Y, Z
    last: Option<Vec2>,
}
impl CursorPosition {
    /// x here is y, y here is z
    pub fn yz(&self) -> Option<Vec2> {
        self.current
    }
    pub fn current(&self) -> Option<Vec3> {
        self.current.map(|yz| Vec3::new(GAME_PLANE, yz.x, yz.y))
    }
    /// x here is y, y here is z
    pub fn yz_last(&self) -> Option<Vec2> {
        self.last
    }
    pub fn last(&self) -> Option<Vec3> {
        self.last.map(|yz| Vec3::new(GAME_PLANE, yz.x, yz.y))
    }
}

fn set_cursor_position(
    camera: Query<(&Camera, &GlobalTransform), With<WorldCamera>>,
    windows: Query<&Window>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let Ok((camera, camera_transform)) = camera.single() else {
        warn!("Camera does not exist for setting the cursor position on the floor!");
        return;
    };

    let window = windows
        .single()
        .expect("A single window. This is unrecoverable!");

    let Some(window_cursor_position) = window.cursor_position() else {
        // can happen if cursor ain't around rn
        cursor_position.current = None;
        return;
    };

    let Ok(ray) = camera.viewport_to_world(camera_transform, window_cursor_position) else {
        cursor_position.current = None;
        return;
    };

    let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Dir3::X)) else {
        cursor_position.current = None;
        return;
    };

    let point = ray.get_point(distance);

    let point2d = Vec2::new(point.x, point.z);

    cursor_position.current = Some(point2d);
    cursor_position.last = Some(point2d);
}
