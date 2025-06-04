use bevy::prelude::*;

use crate::gameplay::sphere::SphereType;

pub struct SpawnSphere {
    pub location: Vec2,
    pub sphere_type: SphereType,
}
impl SpawnSphere {
    pub fn new(location: Vec2, sphere_type: SphereType) -> Self {
        Self {
            location,
            sphere_type,
        }
    }
}

#[macro_export]
macro_rules! sphere {
    ($type:ident, $x:expr, $y:expr) => {
        $crate::gameplay::level::sphere::SpawnSphere::new(
            bevy::prelude::Vec2::new($x, $y),
            $crate::gameplay::sphere::SphereType::$type,
        )
    };
}
