use std::f32::consts::FRAC_PI_2;

mod cancel;
pub use cancel::*;

mod fire;
pub use fire::*;

mod flight_time;
pub use flight_time::*;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{asset_tracking::LoadResource, gameplay::timefreeze::Frozen};

use super::{
    ArrowSet,
    bow::{Bow, pull::PullStrength},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ArrowAssets>()
        .load_resource::<ArrowAssets>();

    app.add_plugins((fire::plugin, cancel::plugin, flight_time::plugin));

    app.add_systems(
        Update,
        update_unfired_arrow_transform.in_set(ArrowSet::UpdateArrow),
    )
    .add_observer(spawn_arrow);
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct ArrowAssets {
    #[dependency]
    pub glowing: Handle<Scene>,
    #[dependency]
    pub normal: Handle<Scene>,
}
impl FromWorld for ArrowAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            glowing: assets.load("models/ArrowGlow.glb#Scene0"),
            normal: assets.load("models/Arrow.glb#Scene0"),
        }
    }
}

#[derive(Event)]
pub struct ReadyArrow;

#[derive(Component, Default)]
#[require(RigidBody = RigidBody::Dynamic)]
#[require(Collider = Collider::capsule(0.1, 3.5))]
#[require(GravityScale = GravityScale(0.))]
pub struct Arrow {
    pub bounces: u8,
}

fn spawn_arrow(_: Trigger<ReadyArrow>, mut commands: Commands, assets: Res<ArrowAssets>) {
    commands.spawn((
        Name::new("Arrow"),
        Arrow::default(),
        SceneRoot(assets.glowing.clone()),
    ));
}

fn update_unfired_arrow_transform(
    mut arrows: Query<&mut Transform, (With<Arrow>, Without<Fired>)>,
    bow: Query<(&Transform, &PullStrength), (With<Bow>, Without<Arrow>)>,
) {
    let Ok((bow, pull_strength)) = bow.single() else {
        return;
    };

    for mut arrow in &mut arrows {
        // since the strength is from 0, 1, that scales from 0 to this number
        const BOW_RIGIDITY: f32 = 3.;
        /// this is how far to translate the arrow to sit on the bow string
        const STRING_OFFSET: f32 = -1.5;
        let sv = pull_strength.strength() * BOW_RIGIDITY;
        let strength_vec = bow.rotation * Vec3::new(sv + STRING_OFFSET, 0., 0.);
        arrow.translation = bow.translation + strength_vec;
        let (z, _, _) = bow.rotation.to_euler(EulerRot::ZXY);
        arrow.rotation = Quat::from_rotation_z(z + FRAC_PI_2);
    }

    //todo
}
