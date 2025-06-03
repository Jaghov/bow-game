use avian3d::prelude::{Collider, CollidingEntities, GravityScale, LockedAxes, RigidBody};
use bevy::{
    color::palettes::{
        css::{BLACK, GREEN, ORANGE, YELLOW},
        tailwind::{BLUE_400, PURPLE_100, PURPLE_900},
    },
    prelude::*,
};

use crate::{Screen, asset_tracking::LoadResource, world::GAME_PLANE};

//use super::particles::ExampleParticles;

pub(super) fn plugin(app: &mut App) {
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

pub enum SphereType {
    Normal,
    Multiplier,
    TimeFreeze,
    Exploder,
    Bouncy,
    Gravity,
    Absorber,
}
#[derive(Component)]
pub struct Sphere;

#[derive(Component)]
pub struct Normal;

#[derive(Component)]
pub struct Multiplier;

#[derive(Component)]
pub struct TimeFreeze;

#[derive(Component)]
pub struct Absorber;

#[derive(Component)]
pub struct Bouncy;

#[derive(Component)]
pub struct GravitySphere;

#[derive(Component)]
pub struct Exploder;

#[derive(Event)]
pub struct SpawnSphere {
    location: Vec2,
    sphere_type: SphereType,
}
impl SpawnSphere {
    pub fn new(location: Vec2, sphere_type: SphereType) -> Self {
        Self {
            location,
            sphere_type,
        }
    }
}

fn spawn_sphere(trigger: Trigger<SpawnSphere>, mut commands: Commands, assets: Res<SphereAssets>) {
    let event = trigger.event();
    let transform = Transform::from_xyz(event.location.x, event.location.y, GAME_PLANE);

    let bundle = (
        Sphere,
        transform,
        Mesh3d(assets.mesh.clone()),
        Collider::sphere(1.),
        RigidBody::Dynamic,
        LockedAxes::default().lock_translation_z(),
        GravityScale(0.),
        CollidingEntities::default(),
    );

    match event.sphere_type {
        SphereType::Normal => {
            commands.spawn((bundle, (Normal, MeshMaterial3d(assets.normal.clone()))))
        }
        SphereType::Multiplier => commands.spawn((
            bundle,
            (Multiplier, MeshMaterial3d(assets.multiplier.clone())),
        )),
        SphereType::TimeFreeze => commands.spawn((
            bundle,
            (TimeFreeze, MeshMaterial3d(assets.time_freeze.clone())),
        )),
        SphereType::Bouncy => {
            commands.spawn((bundle, (Bouncy, MeshMaterial3d(assets.time_freeze.clone()))))
        }
        SphereType::Gravity => commands.spawn((
            bundle,
            (GravitySphere, MeshMaterial3d(assets.time_freeze.clone())),
        )),
        SphereType::Absorber => commands.spawn((
            bundle,
            (Absorber, MeshMaterial3d(assets.time_freeze.clone())),
        )),
        SphereType::Exploder => commands.spawn((
            bundle,
            (Exploder, MeshMaterial3d(assets.time_freeze.clone())),
        )),
    };
    //.observe(particles_on_collision);
}

// fn particles_on_collision(
//     trigger: Trigger<OnCollisionStart>,
//     transforms: Query<&Transform>,
//     assets: Res<ExampleParticles>,
//     mut commands: Commands,
// ) {
//     let event = trigger.event();
//     //let t1 = transforms.get(trigger.target()).unwrap();
//     let t2 = transforms.get(event.body.unwrap()).unwrap();
//     //let avg = (t1.translation + t2.translation);

//     commands.spawn((ParticleEffect::new(assets.0.clone()), *t2));

//     //todo
// }
