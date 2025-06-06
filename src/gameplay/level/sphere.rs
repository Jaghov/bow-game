use avian3d::prelude::Sensor;
use bevy::prelude::*;

use crate::gameplay::sphere::*;

#[derive(Component, Clone, Copy)]
#[allow(dead_code)]
pub enum SphereType {
    Normal,
    Multiplier,
    TimeFreeze,
    Exploder,
    Bouncy,
    Gravity,
    Absorber,
}

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
            $crate::gameplay::level::sphere::SphereType::$type,
        )
    };
}

pub(super) fn spawn_sphere(
    trigger: Trigger<OnAdd, SphereType>,
    mut commands: Commands,
    spheres: Query<&SphereType>,
) {
    let sphere_type = spheres.get(trigger.target()).unwrap();
    let mut ec = commands.entity(trigger.target());
    match sphere_type {
        SphereType::Normal => {
            ec.insert((Name::new("Normal Sphere"), Normal));
        }
        SphereType::Multiplier => {
            ec.insert((Name::new("Multiplier Sphere"), Multiplier, Sensor));
        }
        SphereType::TimeFreeze => {
            ec.insert((Name::new("TimeFreeze Sphere"), TimeFreeze, Sensor));
        }
        SphereType::Bouncy => {
            ec.insert((Name::new("Bouncy Sphere"), Bouncy));
        }
        SphereType::Gravity => {
            ec.insert((Name::new("Gravity Sphere"), GravitySphere));
        }
        SphereType::Absorber => {
            ec.insert((Name::new("Absorber Sphere"), Absorber));
        }
        SphereType::Exploder => {
            ec.insert((Name::new("Exploder Sphere"), Exploder, Sensor));
        }
    }
}
