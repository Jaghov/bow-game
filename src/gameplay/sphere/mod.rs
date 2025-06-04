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

use crate::{
    asset_tracking::LoadResource,
    gameplay::arrow::{Arrow, NockedOn},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        normal::plugin,
        multiplier::plugin,
        timefreeze::plugin,
        exploder::plugin,
    ));

    app.register_type::<SphereAssets>()
        .load_resource::<SphereAssets>();

    app.add_observer(spawn_sphere)
        .add_observer(add_destroyable_sphere);
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct SphereAssets {
    #[dependency]
    pub model: Handle<Scene>,
    #[dependency]
    pub mesh: Handle<Mesh>,
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
#[require(Sphere)]
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

#[derive(Component)]
pub struct Absorber;

#[derive(Component)]
pub struct Bouncy;

#[derive(Component)]
pub struct GravitySphere;

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
            ec.insert((Normal, MeshMaterial3d(assets.normal.clone())));
        }
        SphereType::Multiplier => {
            ec.insert((Multiplier, MeshMaterial3d(assets.multiplier.clone())));
        }
        SphereType::TimeFreeze => {
            ec.insert((TimeFreeze, MeshMaterial3d(assets.time_freeze.clone())));
        }
        SphereType::Bouncy => {
            ec.insert((Bouncy, MeshMaterial3d(assets.bouncy.clone())));
        }
        SphereType::Gravity => {
            ec.insert((GravitySphere, MeshMaterial3d(assets.gravity.clone())));
        }
        SphereType::Absorber => {
            ec.insert((Absorber, MeshMaterial3d(assets.absorber.clone())));
        }
        SphereType::Exploder => {
            ec.insert((Exploder, MeshMaterial3d(assets.exploder.clone())));
        }
    }
}

fn debug_collision(trigger: Trigger<OnCollisionStart>, arrows: Query<&Arrow>) {
    let event = trigger.event();

    info!(
        "Collision event: was arrow? {}",
        arrows.get(event.collider).is_ok()
    );
}

fn despawn_on_arrow(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    arrows: Query<&Arrow, Without<NockedOn>>,
    child_of: Query<&ChildOf>,
) {
    let event = trigger.event();
    if arrows.get(event.collider).is_err() {
        return;
    }
    let parent = child_of.get(trigger.target()).unwrap().parent();

    commands.entity(parent).trigger(DestroySphere);
}

fn add_destroyable_sphere(trigger: Trigger<OnAdd, Sphere>, mut commands: Commands) {
    commands.entity(trigger.target()).observe(destroy_sphere);
}

#[derive(Event)]
pub struct DestroySphere;
// listener should ONLY be on the Sphere component.
fn destroy_sphere(trigger: Trigger<DestroySphere>, mut commands: Commands) {
    // this will make the thing break into a million pieces.
    // TODO
    commands.entity(trigger.target()).despawn();
}

// fn spawn_sphere(trigger: Trigger<SpawnSphere>, mut commands: Commands, assets: Res<SphereAssets>) {
//     let event = trigger.event();
//     let transform = Transform::from_xyz(event.location.x, event.location.y, GAME_PLANE);

//     let bundle = (
//         Sphere,
//         transform,
//         Mesh3d(assets.mesh.clone()),
//         Collider::sphere(1.),
//         RigidBody::Dynamic,
//         LockedAxes::default().lock_translation_z(),
//         GravityScale(0.),
//         CollidingEntities::default(),
//     );

//     match event.sphere_type {
//         SphereType::Normal => commands.spawn((normal(&assets), transform)),
//         SphereType::Multiplier => commands.spawn((multiplier(&assets), transform)),
//         SphereType::TimeFreeze => commands.spawn((timefreeze(&assets), transform)),
//         SphereType::Bouncy => {
//             commands.spawn((bundle, (Bouncy, MeshMaterial3d(assets.time_freeze.clone()))))
//         }
//         SphereType::Gravity => commands.spawn((
//             bundle,
//             (GravitySphere, MeshMaterial3d(assets.time_freeze.clone())),
//         )),
//         SphereType::Absorber => commands.spawn((
//             bundle,
//             (Absorber, MeshMaterial3d(assets.time_freeze.clone())),
//         )),
//         SphereType::Exploder => commands.spawn((exploder(&assets), transform)),
//     };
// }
