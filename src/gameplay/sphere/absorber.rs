use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    gameplay::sphere::{Bouncy, Exploder, Multiplier, Sphere},
    third_party::avian3d::GameLayer,
    world::GAME_PLANE,
};

/// WIP, need to fix a few systems
#[derive(Component)]
#[require(Sphere)]
pub struct Absorber;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_absorber);
}
fn insert_absorber(trigger: Trigger<OnAdd, Absorber>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert((
            CollisionLayers::new(
                GameLayer::Sphere,
                [GameLayer::Arrow, GameLayer::Sphere, GameLayer::Walls],
            ),
            Collider::sphere(1.),
            Restitution::PERFECTLY_ELASTIC,
            CollisionEventsEnabled,
        ))
        .observe(super::debug_collision)
        .observe(absorb_property::<Bouncy>)
        .observe(absorb_property::<Multiplier>)
        .observe(absorb_property::<Exploder>);
}

fn absorb_property<Prop>(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    spheres: Query<&MeshMaterial3d<StandardMaterial>, (With<Sphere>, With<Prop>)>,
    absorbers_without_prop: Query<
        &MeshMaterial3d<StandardMaterial>,
        (Without<Prop>, With<Absorber>),
    >,
    colliders: Query<&ColliderOf>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) where
    Prop: Component + Default,
{
    info!("inserting property on absorber");
    let Ok(absorber_material) = absorbers_without_prop.get(trigger.target()) else {
        return;
    };
    let Ok(collider) = colliders.get(trigger.event().collider) else {
        return;
    };
    let Ok(collider_material) = spheres.get(collider.body) else {
        return;
    };

    let collider_color = {
        let material = materials.get(collider_material).unwrap();
        material.base_color
    };

    let absorber_material = materials.get_mut(absorber_material).unwrap();

    absorber_material.base_color.mix_assign(collider_color, 0.3);

    commands.entity(trigger.target()).insert(Prop::default());
}

// fn on_multiply(
//     trigger: Trigger<ShouldMultiply>,
//     mut commands: Commands,
//     bouncy_balls: Query<(&Transform, &LinearVelocity), With<Absorber>>,
// ) {
//     info!("in absorber on multiply");
//     let event = trigger.event();
//     let Ok((arrow_trn, lvel)) = bouncy_balls.get(trigger.target()) else {
//         warn!("Bouncy ball was commanded to multiply, but its required components were not found!");
//         return;
//     };

//     let multiply_origin = event.local_point.with_z(GAME_PLANE);

//     for rotation_offset in &event.rot_offset {
//         let quatrot = Quat::from_rotation_z(*rotation_offset);
//         let rotation = arrow_trn.rotation * Quat::from_rotation_z(*rotation_offset);

//         let velocity = quatrot * lvel.0;
//         let offset = velocity.normalize() * 2.2;

//         let transform = Transform::from_translation(multiply_origin + offset)
//             .with_rotation(rotation)
//             .with_scale(arrow_trn.scale);

//         commands.spawn((SphereType::Bouncy, transform, LinearVelocity(velocity)));
//     }
// }
