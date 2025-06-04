use avian3d::prelude::Collider;
use bevy::prelude::*;

use crate::world::BLOCK_LEN;
// this is a builder.
pub struct WallBuilder {
    pub mesh: Cuboid,
    pub collider: Collider,
    pub transform: Transform, //start:
}
impl WallBuilder {
    // note that these values are not multiplied by `BLOCK_LEN`.
    pub fn horizontal(y: f32, start_x: f32, end_x: f32) -> Self {
        let (near_x, far_x) = if start_x < end_x {
            (start_x, end_x)
        } else {
            (end_x, start_x)
        };

        let length = far_x - near_x;

        let middle_x = (far_x + near_x) * 0.5;

        let mesh = Cuboid::new(length, BLOCK_LEN, BLOCK_LEN);

        let collider = Collider::cuboid(length, BLOCK_LEN, BLOCK_LEN);

        let transform = Transform::from_xyz(middle_x, y, 0.);
        Self {
            mesh,
            collider,
            transform,
        }
    }
    pub fn vertical(x: f32, start_y: f32, end_y: f32) -> Self {
        let (near_y, far_y) = if start_y < end_y {
            (start_y, end_y)
        } else {
            (end_y, start_y)
        };

        let length = far_y - near_y;

        let middle_y = (far_y + near_y) * 0.5;

        let mesh = Cuboid::new(BLOCK_LEN, length, BLOCK_LEN);

        let collider = Collider::cuboid(BLOCK_LEN, length, BLOCK_LEN);

        let transform = Transform::from_xyz(x, middle_y, 0.);
        Self {
            mesh,
            collider,
            transform,
        }
    }
}

/// Values are automatically multiplied by block len
#[macro_export]
macro_rules! vert {
    ($x:literal, $start_y:literal, $end_y:literal) => {
        $crate::gameplay::level::wall::WallBuilder::vertical(
            $crate::world::BLOCK_LEN * $x,
            $crate::world::BLOCK_LEN * $start_y,
            $crate::world::BLOCK_LEN * $end_y,
        )
    };
}
/// Values are automatically multiplied by block len
#[macro_export]
macro_rules! horz {
    ($x:literal, $start_y:literal, $end_y:literal) => {
        $crate::gameplay::level::wall::WallBuilder::horizontal(
            $crate::world::BLOCK_LEN * $x,
            $crate::world::BLOCK_LEN * $start_y,
            $crate::world::BLOCK_LEN * $end_y,
        )
    };
}
