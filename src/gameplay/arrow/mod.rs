use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use crate::asset_tracking::LoadResource;

use super::{
    ArrowSet, GAME_PLANE, GameLoadState,
    bow::{Bow, pull::PullStrength},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ArrowAssets>()
        .load_resource::<ArrowAssets>();

    app.add_systems(OnEnter(GameLoadState::Loaded), spawn_debug_arrows)
        .add_systems(
            Update,
            update_unfired_arrow_transform.in_set(ArrowSet::UpdateArrow),
        )
        .add_observer(spawn_arrow)
        .add_observer(fire_arrow)
        .add_observer(cancel_arrow);
    //todo
}

#[derive(Resource, Asset, Reflect, Clone)]
struct ArrowAssets {
    #[dependency]
    glowing: Handle<Scene>,
    #[dependency]
    normal: Handle<Scene>,
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
fn spawn_debug_arrows(mut commands: Commands, assets: Res<ArrowAssets>) {
    commands.spawn((
        Transform::from_xyz(-5., 0., GAME_PLANE),
        SceneRoot(assets.glowing.clone()),
    ));
    commands.spawn((
        Transform::from_xyz(5., 0., GAME_PLANE),
        SceneRoot(assets.normal.clone()),
    ));
}

#[derive(Event)]
pub struct ReadyArrow;

#[derive(Event)]
pub struct CancelArrow;

#[derive(Event)]
pub struct FireArrow;

#[derive(Component)]
pub struct Arrow;

fn spawn_arrow(_: Trigger<ReadyArrow>, mut commands: Commands, assets: Res<ArrowAssets>) {
    commands.spawn((
        Arrow,
        SceneRoot(assets.glowing.clone()),
        Transform::default(),
    ));
    //todo
}
fn fire_arrow(
    _: Trigger<FireArrow>,
    mut commands: Commands,
    arrows: Query<Entity, (With<Arrow>, Without<Fired>)>,
) {
    for arrow in arrows {
        commands.entity(arrow).despawn();
    }
}

fn cancel_arrow(
    _: Trigger<CancelArrow>,
    mut commands: Commands,
    arrows: Query<Entity, (With<Arrow>, Without<Fired>)>,
) {
    for arrow in arrows {
        commands.entity(arrow).despawn();
    }
}

#[derive(Component)]
pub struct Fired;
fn update_unfired_arrow_transform(
    mut arrows: Query<&mut Transform, (With<Arrow>, Without<Fired>)>,
    bow: Query<(&Transform, &PullStrength), (With<Bow>, Without<Arrow>)>,
) {
    let Ok((bow, pull_strength)) = bow.single() else {
        return;
    };

    for mut arrow in &mut arrows {
        let mut translation = bow.translation;
        const MULT: f32 = 3.;
        /// this is how far to translate the arrow to sit on the bow string
        const STRING_OFFSET: f32 = -1.5;
        let sv = pull_strength.strength() * MULT;

        let strength_vec = bow.rotation * Vec3::new(sv + STRING_OFFSET, 0., 0.);

        arrow.translation = bow.translation + strength_vec;
        let (z, _, _) = bow.rotation.to_euler(EulerRot::ZXY);
        arrow.rotation = Quat::from_rotation_z(z + FRAC_PI_2);
    }

    //todo
}
