use avian3d::prelude::Collider;
use bevy::prelude::*;

use crate::world::BLOCK_LEN;

#[allow(dead_code)]
pub enum WallMesh {
    Cuboid(Cuboid),
    Cylinder(Extrusion<Circle>),
}

// this is a builder.
pub struct WallBuilder {
    pub mesh: WallMesh,
    pub collider: Collider,
    pub transform: Transform, //start:
}
#[allow(dead_code)]
impl WallBuilder {
    // note that these values are not multiplied by `BLOCK_LEN`.
    pub fn horizontal(y: f32, start_x: f32, end_x: f32) -> Self {
        let length = (end_x - start_x).abs();

        let middle_x = (start_x + end_x) * 0.5;

        let mesh = Cuboid::new(length, BLOCK_LEN, BLOCK_LEN);

        let collider = Collider::cuboid(length, BLOCK_LEN, BLOCK_LEN);

        let transform = Transform::from_xyz(middle_x, y, 0.);
        Self {
            mesh: WallMesh::Cuboid(mesh),
            collider,
            transform,
        }
    }
    pub fn vertical(x: f32, start_y: f32, end_y: f32) -> Self {
        let length = (end_y - start_y).abs();

        let middle_y = (start_y + end_y) * 0.5;

        let mesh = Cuboid::new(BLOCK_LEN, length, BLOCK_LEN);

        let collider = Collider::cuboid(BLOCK_LEN, length, BLOCK_LEN);

        let transform = Transform::from_xyz(x, middle_y, 0.);
        Self {
            mesh: WallMesh::Cuboid(mesh),
            collider,
            transform,
        }
    }
    pub fn pole(radius: f32, x: f32, y: f32) -> Self {
        let mesh = Extrusion::new(Circle::new(radius), BLOCK_LEN);

        let collider = Collider::cylinder(radius, 1.);

        let transform = Transform::from_xyz(x, y, 0.);
        Self {
            mesh: WallMesh::Cylinder(mesh),
            collider,
            transform,
        }
    }
    pub fn block(x_len: f32, y_len: f32, x: f32, y: f32) -> Self {
        let mesh = Cuboid::new(x_len, y_len, BLOCK_LEN);

        let collider = Collider::cuboid(x_len, y_len, BLOCK_LEN);

        let transform = Transform::from_xyz(x, y, 0.);
        Self {
            mesh: WallMesh::Cuboid(mesh),
            collider,
            transform,
        }
    }
    pub fn block_rot(x_len: f32, y_len: f32, x: f32, y: f32, rot: f32) -> Self {
        let mesh = Cuboid::new(x_len, y_len, BLOCK_LEN);

        let collider = Collider::cuboid(x_len, y_len, BLOCK_LEN);

        let transform = Transform::from_xyz(x, y, 0.).with_rotation(Quat::from_rotation_z(rot));
        Self {
            mesh: WallMesh::Cuboid(mesh),
            collider,
            transform,
        }
    }
}

/// Values are automatically multiplied by block len
#[macro_export]
macro_rules! vert {
    ($x:literal, $start_y:literal, $end_y:literal) => {
        #[allow(clippy::neg_multiply)]
        $crate::gameplay::level::wall::WallBuilder::vertical(
            $crate::world::BLOCK_LEN * $x,
            $crate::world::BLOCK_LEN * $start_y - $crate::world::BLOCK_LEN * 0.5,
            $crate::world::BLOCK_LEN * $end_y + $crate::world::BLOCK_LEN * 0.5,
        )
    };
}
/// Values are automatically multiplied by block len
#[macro_export]
macro_rules! horz {
    ($x:literal, $start_y:literal, $end_y:literal) => {
        #[allow(clippy::neg_multiply)]
        $crate::gameplay::level::wall::WallBuilder::horizontal(
            $crate::world::BLOCK_LEN * $x,
            $crate::world::BLOCK_LEN * $start_y - $crate::world::BLOCK_LEN * 0.5,
            $crate::world::BLOCK_LEN * $end_y + $crate::world::BLOCK_LEN * 0.5,
        )
    };
}

#[derive(Component)]
pub struct Walls;
