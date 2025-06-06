use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    gameplay::{level::LevelState, sphere::Sphere},
    third_party::avian3d::GameLayer,
};

#[derive(Component)]
#[require(Sphere)]
pub struct GravitySphere;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_gravity_sphere).add_systems(
        FixedUpdate,
        apply_gravity_forces_to_spheres.run_if(in_state(LevelState::Playing)),
    );
}
fn insert_gravity_sphere(trigger: Trigger<OnAdd, GravitySphere>, mut commands: Commands) {
    commands
        .spawn((
            CollisionLayers::new(
                GameLayer::Sphere,
                [GameLayer::Arrow, GameLayer::Sphere, GameLayer::Walls],
            ),
            Collider::sphere(1.),
            Restitution::PERFECTLY_ELASTIC,
            CollisionEventsEnabled,
            ChildOf(trigger.target()),
        ))
        .observe(super::debug_collision);

    commands.entity(trigger.target()).insert(Dominance(1));
}
/// the min distance for the gravity sphere to emit a force
const ATTRACTION_RADIUS: f32 = 10.;

fn apply_gravity_forces_to_spheres(
    gravity_spheres: Query<(Entity, &Position), With<GravitySphere>>,
    colliders: Query<&ColliderOf>,
    positions: Query<&Position>,
    mut forces: Query<&mut ExternalForce>,
    spatial_query: SpatialQuery,
) {
    for (sphere, position) in gravity_spheres {
        let shape = Collider::sphere(ATTRACTION_RADIUS);
        let origin = position.0;
        let rotation = Quat::default();
        let filter = SpatialQueryFilter::from_mask([GameLayer::Sphere, GameLayer::Arrow]);
        let hits = spatial_query.shape_intersections(&shape, origin, rotation, &filter);

        for hit in hits {
            let Ok(collider) = colliders.get(hit) else {
                continue;
            };
            let body = collider.body;
            if body == sphere {
                continue;
            }
            let Ok(other_sphere_position) = positions.get(body) else {
                continue;
            };
            let Ok(mut force) = forces.get_mut(body) else {
                warn!("couldn't get external force for other body");
                continue;
            };
            force.persistent = false;

            let direction = position.0 - other_sphere_position.0;
            let len = ATTRACTION_RADIUS.powi(2) - direction.length_squared();
            force.apply_force(direction.normalize() * len);
        }
    }
}
