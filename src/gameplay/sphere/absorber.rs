use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    gameplay::sphere::{Bouncy, Exploder, Multiplier, MustMark, Sphere, SphereAssets},
    third_party::avian3d::GameLayer,
};

/// WIP, need to fix a few systems
#[derive(Component)]
#[require(Sphere)]
#[require(MustMark)]
pub struct Absorber;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(insert_absorber);
}
fn insert_absorber(
    trigger: Trigger<OnAdd, Absorber>,
    mut commands: Commands,
    assets: Res<SphereAssets>,
) {
    commands
        .entity(trigger.target())
        .insert((
            CollisionLayers::new(
                GameLayer::Sphere,
                [GameLayer::Arrow, GameLayer::Sphere, GameLayer::Walls],
            ),
            MeshMaterial3d(assets.absorber.clone()),
            Restitution::PERFECTLY_ELASTIC,
        ))
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
        let Some(material) = materials.get(collider_material) else {
            return;
        };
        material.base_color
    };

    let Some(absorber_material) = materials.get_mut(absorber_material) else {
        return;
    };

    absorber_material.base_color.mix_assign(collider_color, 0.3);

    commands.entity(trigger.target()).insert(Prop::default());
}
