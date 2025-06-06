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
        .load_resource::<SphereAssets>();

    app.add_observer(spawn_sphere);
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct SphereAssets {
    #[dependency]
    pub model: Handle<Scene>,
    #[dependency]
    pub mesh: Handle<Mesh>,
    #[dependency]
    pub gibs: Handle<Scene>,
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
            normal,
            multiplier,
            time_freeze,
            absorber,
            bouncy,
            gravity,
            exploder,
        }
    }
}

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
#[derive(Component, Default)]
#[require(RigidBody = RigidBody::Dynamic)]
#[require(LockedAxes = LockedAxes::new().lock_translation_z())]
pub struct Sphere;

fn spawn_sphere(
    trigger: Trigger<OnAdd, SphereType>,
    mut commands: Commands,
    spheres: Query<&SphereType>,
    assets: Res<SphereAssets>,
) {
    let sphere_type = spheres.get(trigger.target()).unwrap();
    let mut ec = commands.entity(trigger.target());
    ec.insert(Mesh3d(assets.mesh.clone()));
    match sphere_type {
        SphereType::Normal => {
            ec.insert((
                Name::new("Normal Sphere"),
                Normal,
                MeshMaterial3d(assets.normal.clone()),
            ));
        }
        SphereType::Multiplier => {
            ec.insert((
                Name::new("Multiplier Sphere"),
                Multiplier,
                MeshMaterial3d(assets.multiplier.clone()),
            ));
        }
        SphereType::TimeFreeze => {
            ec.insert((
                Name::new("TimeFreeze Sphere"),
                TimeFreeze,
                MeshMaterial3d(assets.time_freeze.clone()),
            ));
        }
        SphereType::Bouncy => {
            ec.insert((
                Name::new("Bouncy Sphere"),
                Bouncy,
                MeshMaterial3d(assets.bouncy.clone()),
            ));
        }
        SphereType::Gravity => {
            ec.insert((
                Name::new("Gravity Sphere"),
                GravitySphere,
                MeshMaterial3d(assets.gravity.clone()),
            ));
        }
        SphereType::Absorber => {
            ec.insert((
                Name::new("Absorber Sphere"),
                Absorber,
                MeshMaterial3d(assets.absorber.clone()),
            ));
        }
        SphereType::Exploder => {
            ec.insert((
                Name::new("Exploder Sphere"),
                Exploder,
                MeshMaterial3d(assets.exploder.clone()),
            ));
        }
    }
}

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
    arrows: Query<(), (With<Arrow>, Without<NockedOn>)>,
    colliders: Query<&ColliderOf>,
) {
    let event = trigger.event();
    let Ok(collider) = colliders.get(event.collider) else {
        return;
    };
    if arrows.get(collider.body).is_err() {
        warn!("collided, not with arrow");
        return;
    }
    let parent = colliders.get(trigger.target()).unwrap().body;

    commands.entity(parent).trigger(DestroySphere);
}

fn despawn_on_bouncyball_collision(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    spheres: Query<(), (With<Sphere>, With<Bouncy>)>,
    colliders: Query<&ColliderOf>,
) {
    let event = trigger.event();
    let Ok(collider) = colliders.get(event.collider) else {
        return;
    };
    if spheres.get(collider.body).is_err() {
        warn!("collided, not with sphere");
        return;
    }
    let parent = colliders.get(trigger.target()).unwrap().body;

    commands.entity(parent).trigger(DestroySphere);
}
