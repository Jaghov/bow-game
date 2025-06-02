use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_hanabi::ParticleEffect;

use crate::asset_tracking::LoadResource;
mod effects;

use super::{GAME_PLANE, GameLoadState, particles::ExampleParticles};
use bevy::{
    color::palettes::{
        css::{RED, WHITE},
        tailwind::{BLUE_200, BLUE_400},
    },
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TargetAssets>()
        .load_resource::<TargetAssets>();
    app.add_systems(OnEnter(GameLoadState::Loaded), (spawn_sample_level,))
        .add_observer(spawn_target);
    app.add_plugins(effects::plugin);
}
#[derive(Resource, Asset, Reflect, Clone)]
struct TargetAssets {
    #[dependency]
    model: Handle<Mesh>,
    normal: Handle<StandardMaterial>,
    multiplier: Handle<StandardMaterial>,
    time_freeze: Handle<StandardMaterial>,
}
impl FromWorld for TargetAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let model = assets.load(
            GltfAssetLabel::Primitive {
                mesh: 0,
                primitive: 0,
            }
            .from_asset("models/sphere.glb"),
        );
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let normal_material = StandardMaterial {
            base_color: Color::srgb(0.7, 0.7, 1.0),
            // specular_tint: Color::from(Srgba::RED),
            reflectance: 0.3,
            specular_transmission: 0.9,
            diffuse_transmission: 1.0,
            thickness: 1.6,
            ior: 1.5,
            perceptual_roughness: 0.12,
            ..Default::default()
        };

        let normal = materials.add(normal_material.clone());

        let multiplier = materials.add(StandardMaterial {
            base_color: RED.into(),
            ..normal_material.clone()
        });
        let time_freeze = materials.add(StandardMaterial {
            base_color: BLUE_400.into(),
            ..normal_material.clone()
        });

        Self {
            model,
            normal,
            multiplier,
            time_freeze,
        }
    }
}

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

pub enum SphereType {
    Normal,
    Multiplier,
    TimeFreeze,
}
#[derive(Component)]
struct Target;
#[derive(Component)]
struct Normal;
#[derive(Component)]
struct Multiplier;
#[derive(Component)]
struct TimeFreeze;

fn spawn_sample_level(mut commands: Commands) {
    commands.trigger(SpawnSphere::new(Vec2::new(4., 5.), SphereType::Normal));
    commands.trigger(SpawnSphere::new(Vec2::new(0., 5.), SphereType::Multiplier));
    commands.trigger(SpawnSphere::new(Vec2::new(-4., 5.), SphereType::TimeFreeze));
}

fn spawn_target(trigger: Trigger<SpawnSphere>, mut commands: Commands, assets: Res<TargetAssets>) {
    let event = trigger.event();
    let transform = Transform::from_xyz(event.location.x, event.location.y, GAME_PLANE);

    let bundle = (
        Target,
        transform,
        Mesh3d(assets.model.clone()),
        Collider::sphere(1.),
        Sensor,
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
    }
    .observe(particles_on_collision);
}

fn particles_on_collision(
    trigger: Trigger<OnCollisionStart>,
    transforms: Query<&Transform>,
    assets: Res<ExampleParticles>,
    mut commands: Commands,
) {
    let event = trigger.event();
    //let t1 = transforms.get(trigger.target()).unwrap();
    let t2 = transforms.get(event.body.unwrap()).unwrap();
    //let avg = (t1.translation + t2.translation);

    commands.spawn((ParticleEffect::new(assets.0.clone()), *t2));

    //todo
}
