use std::path::Path;

use avian3d::prelude::*;
use bevy::{
    color::palettes::{
        css::{GREEN, ORANGE, YELLOW},
        tailwind::BLUE_400,
    },
    prelude::*,
};

mod normal;
pub use normal::*;

mod exploder;
pub use exploder::*;

mod multiplier;
pub use multiplier::*;

mod timefreeze;
pub use timefreeze::*;

mod bouncy;
pub use bouncy::*;

mod destroy;
pub use destroy::*;

mod gravity;
pub use gravity::*;

mod absorber;
pub use absorber::*;

use crate::{
    asset_tracking::LoadResource,
    gameplay::arrow::{Arrow, NockedOn},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        normal::plugin,
        multiplier::plugin,
        timefreeze::plugin,
        absorber::plugin,
        exploder::plugin,
        bouncy::plugin,
        destroy::plugin,
        gravity::plugin,
    ));

    app.register_type::<SphereAssets>()
        .load_resource::<SphereAssets>()
        .add_observer(add_sphere_mesh);
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct SphereAssets {
    #[dependency]
    pub model: Handle<Scene>,
    #[dependency]
    pub mesh: Handle<Mesh>,
    #[dependency]
    pub gibs: Handle<Scene>,
    pub break_sfx: Handle<AudioSource>,
    pub normal: Handle<StandardMaterial>,
    pub multiplier: Handle<StandardMaterial>,
    pub time_freeze: Handle<StandardMaterial>,
    pub absorber: Handle<StandardMaterial>,
    pub bouncy: Handle<StandardMaterial>,
    pub gravity: Handle<StandardMaterial>,
    pub exploder: Handle<StandardMaterial>,
}

impl FromWorld for SphereAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let model = assets.load("models/sph.glb#Scene0");
        let mesh = assets.load("models/sph.glb#Mesh0/Primitive0");
        let gibs = assets.load("models/glass_fractured.glb#Scene0");
        let break_sfx = assets.load(Path::new("audio/sfx/GlassBreakSFX.flac"));
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

        let base = StandardMaterial {
            base_color: Color::srgb(0.7, 0.7, 1.0),
            // specular_tint: Color::from(Srgba::RED),
            reflectance: 1.,
            specular_transmission: 0.90,
            diffuse_transmission: 0.5,
            thickness: 0.6,
            ior: 1.5,
            perceptual_roughness: 0.4,
            ..Default::default()
        };

        let normal = materials.add(base.clone());

        let multiplier = materials.add(StandardMaterial {
            base_color: ORANGE.into(),
            ..default()
        });

        let time_freeze = materials.add(StandardMaterial {
            base_color: BLUE_400.into(),
            emissive: LinearRgba::new(0.0, 0., 1., 1.),
            ..default()
        });

        let absorber = materials.add(StandardMaterial {
            base_color: GREEN.into(),
            reflectance: 1.,
            specular_transmission: 0.90,
            diffuse_transmission: 0.5,
            thickness: 0.6,
            ior: 1.5,
            perceptual_roughness: 0.4,
            ..default()
        });

        let bouncy = materials.add(StandardMaterial {
            base_color: YELLOW.into(),
            ..default()
        });

        let gravity = materials.add(StandardMaterial {
            base_color: Color::BLACK,
            emissive: Color::BLACK.into(),
            unlit: true,
            // specular_tint: Color::from(Srgba::RED),
            reflectance: 0.,
            metallic: 0.,
            specular_transmission: 0.0,
            diffuse_transmission: 0.0,
            thickness: 5.,
            ior: 1.,
            perceptual_roughness: 1.0,
            ..default()
        });

        let exploder = materials.add(StandardMaterial {
            //base_color: RED.into(),
            emissive: Srgba::rgb(5., 0., 0.).into(),
            ..default()
        });
        Self {
            model,
            mesh,
            gibs,
            break_sfx,
            normal,
            absorber,
            multiplier,
            time_freeze,
            bouncy,
            gravity,
            exploder,
        }
    }
}

#[derive(Component, Default)]
#[require(RigidBody = RigidBody::Dynamic)]
#[require(LockedAxes = LockedAxes::ROTATION_LOCKED.lock_translation_z())]
#[require(Collider = Collider::sphere(1.))]
#[require(CollisionEventsEnabled)]
pub struct Sphere;

fn add_sphere_mesh(
    trigger: Trigger<OnAdd, Sphere>,
    mut commands: Commands,
    assets: Res<SphereAssets>,
) {
    commands
        .entity(trigger.target())
        .try_insert(Mesh3d(assets.mesh.clone()));
}

#[allow(dead_code)]
fn debug_collision(
    trigger: Trigger<OnCollisionStart>,
    arrows: Query<&Arrow>,
    spheres: Query<&Sphere>,
    children: Query<&ChildOf>,
    colliders: Query<&ColliderOf>,
) {
    let event = trigger.event();
    let mut message = "COLLISION EVENT:".to_string();

    let target = trigger.target();

    let mut sph1 = None;

    if let Ok(child) = children.get(target) {
        message.push_str(" is_child");
        if spheres.get(child.parent()).is_ok() {
            sph1 = Some(child.parent());
            message.push_str("_of_sphere");
        } else {
            message.push_str("_of_non_sphere");
        }
    } else {
        message.push_str(" not_child");
    }

    let mut sph2 = None;

    let collider = event.collider;
    if arrows.get(collider).is_ok() {
        message.push_str(" collider_is_arrow");
    } else if spheres.get(collider).is_ok() {
        message.push_str(" collider_is_sphere");
    } else if let Ok(collider) = colliders.get(collider) {
        message.push_str(" collider_body");
        if arrows.get(collider.body).is_ok() {
            message.push_str("_is_arrow");
        } else if spheres.get(collider.body).is_ok() {
            sph2 = Some(collider.body);
            message.push_str("_is_sphere");
        } else {
            message.push_str("_is_unknown");
        }
    } else {
        message.push_str(" collider_is_unknown");
    }
    match (sph1, sph2) {
        (Some(sph1), Some(sph2)) if sph1 == sph2 => {
            message.push_str(" self_collision");
        }
        (Some(_), Some(_)) => {
            message.push_str(" not_self_collision");
        }
        _ => {}
    }

    info!("{}", message);
}

fn despawn_on_arrow_collision(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    absorbers: Query<(), With<Absorber>>,
    arrows: Query<(), (With<Arrow>, Without<NockedOn>)>,
    colliders: Query<&ColliderOf>,
) {
    if absorbers.get(trigger.target()).is_ok() {
        return;
    };

    let event = trigger.event();
    let Ok(collider) = colliders.get(event.collider) else {
        return;
    };
    if arrows.get(collider.body).is_err() {
        return;
    }
    let Ok(sphere_collider) = colliders.get(trigger.target()) else {
        return;
    };

    commands.entity(sphere_collider.body).trigger(DestroySphere);
}

fn despawn_on_bouncyball_collision(
    trigger: Trigger<OnCollisionStart>,
    absorbers: Query<(), With<Absorber>>,
    mut commands: Commands,
    spheres: Query<(), (With<Sphere>, With<Bouncy>)>,
    colliders: Query<&ColliderOf>,
) {
    if absorbers.get(trigger.target()).is_ok() {
        return;
    };
    let event = trigger.event();
    let Ok(collider) = colliders.get(event.collider) else {
        return;
    };
    if spheres.get(collider.body).is_err() {
        return;
    }
    let parent = colliders.get(trigger.target()).unwrap().body;

    commands.entity(parent).trigger(DestroySphere);
}

// simple observer that will just handle the hit by explosion
// event by starting despawn
fn despawn_on_hit_by_explosion(
    trigger: Trigger<HitByExplosion>,
    absorbers: Query<(), With<Absorber>>,
    mut commands: Commands,
) {
    if absorbers.get(trigger.target()).is_ok() {
        return;
    };

    commands.entity(trigger.target()).trigger(DestroySphere);
}
