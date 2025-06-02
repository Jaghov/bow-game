use avian3d::prelude::{Collider, CollidingEntities, GravityScale, LockedAxes, RigidBody};
use bevy::{color::palettes::tailwind::BLUE_400, prelude::*};

use crate::{Screen, asset_tracking::LoadResource, world::GAME_PLANE};

//use super::particles::ExampleParticles;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<SphereAssets>()
        .load_resource::<SphereAssets>();

    app.add_systems(OnEnter(Screen::Gameplay), (spawn_sample_level,))
        .add_observer(spawn_sphere);
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
}

impl FromWorld for SphereAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let model = assets.load("models/sph.glb#Scene0");
        let mesh = assets.load("models/sph.glb#Mesh0/Primitive0");
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let normal = materials.add(StandardMaterial {
            base_color: Color::srgb(0.7, 0.7, 1.0),
            // specular_tint: Color::from(Srgba::RED),
            reflectance: 0.3,
            specular_transmission: 0.95,
            diffuse_transmission: 1.0,
            thickness: 0.6,
            ior: 1.5,
            perceptual_roughness: 0.12,
            ..Default::default()
        });

        //let red = LinearRgba::from(RED).with_alpha(0.1);
        let emissive = Srgba::rgb(5., 0., 0.);
        let multiplier = materials.add(StandardMaterial {
            //base_color: RED.into(),
            emissive: emissive.into(),
            ..default()
        });
        let time_freeze = materials.add(StandardMaterial {
            base_color: BLUE_400.into(),
            ..default()
        });

        Self {
            model,
            mesh,
            normal,
            multiplier,
            time_freeze,
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
struct Sphere;
#[derive(Component)]
struct Normal;
#[derive(Component)]
struct Multiplier;
#[derive(Component)]
struct TimeFreeze;
#[derive(Component)]
struct Absorber;
#[derive(Component)]
struct Bouncy;
#[derive(Component)]
struct GravitySphere;
#[derive(Component)]
struct Exploder;

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

fn spawn_sample_level(mut commands: Commands) {
    commands.trigger(SpawnSphere::new(Vec2::new(4., 5.), SphereType::Normal));
    commands.trigger(SpawnSphere::new(Vec2::new(0., 5.), SphereType::Multiplier));
    commands.trigger(SpawnSphere::new(Vec2::new(-4., 5.), SphereType::TimeFreeze));
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
