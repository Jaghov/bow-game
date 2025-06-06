use std::{f32::consts::FRAC_PI_2, time::Duration};

use avian3d::prelude::*;
use bevy::{
    color::palettes::css::{ORANGE, RED, YELLOW},
    prelude::*,
};
use bevy_trauma_shake::Shake;

use crate::{
    gameplay::{
        GameSet, GameState,
        arrow::NockedOn,
        sphere::{DestroySphere, Sphere},
    },
    third_party::avian3d::GameLayer,
};

const EXPLOSION_RADIUS: f32 = 8.;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ExploderAssets>();
    app.add_observer(insert_exploder)
        .add_systems(
            Update,
            tick_explosion
                .in_set(GameSet::TickTimers)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (animate_indicator, explode)
                .in_set(GameSet::Update)
                .run_if(in_state(GameState::Playing)),
        );
}

#[derive(Resource)]
struct ExploderAssets {
    torus: Handle<Mesh>,
}
impl FromWorld for ExploderAssets {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();

        let torus = meshes.add(Extrusion::new(
            Annulus::new(EXPLOSION_RADIUS - 0.5, EXPLOSION_RADIUS),
            0.2,
        ));

        Self { torus }
    }
}

#[derive(Component)]
#[require(Sphere)]
pub struct Exploder;

fn insert_exploder(trigger: Trigger<OnAdd, Exploder>, mut commands: Commands) {
    info!("observed new normal insert");

    commands
        .entity(trigger.target())
        .insert((
            CollisionLayers::new(
                GameLayer::Sphere,
                [GameLayer::ArrowSensor, GameLayer::Sphere],
            ),
            Collider::sphere(1.),
            Sensor,
            CollisionEventsEnabled,
        ))
        .observe(super::debug_collision)
        .observe(light_fuse_on_collision);

    commands.entity(trigger.target()).observe(light_fuse);
}

fn on_hit_by_explosion(trigger: Trigger<HitByExplosion>, mut commands: Commands) {
    commands.trigger_targets(LightFuse(1), trigger.target());
}

#[derive(Component, Debug)]
struct Fuse {
    timer: Timer,
    countdown: usize,
}

impl Fuse {
    fn new(ticks: usize) -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
            countdown: ticks,
        }
    }
}

#[derive(Event)]
struct LightFuse(usize);

#[derive(Component)]
struct Indicator(Entity);

fn indicator(assets: &ExploderAssets, materials: &mut Assets<StandardMaterial>) -> impl Bundle {
    (
        Mesh3d(assets.torus.clone()),
        MeshMaterial3d(materials.add(Color::from(YELLOW))),
        Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2)),
    )
}

fn light_fuse_on_collision(
    trigger: Trigger<OnCollisionStart>,
    ignore: Query<(), With<NockedOn>>,
    mut commands: Commands,
) {
    if ignore.get(trigger.event().collider).is_ok() {
        return;
    }
    commands.trigger_targets(LightFuse(3), trigger.target());
}

fn light_fuse(
    trigger: Trigger<LightFuse>,
    mut commands: Commands,
    mut exploders: Query<(Entity, Has<Fuse>), With<Exploder>>,
    assets: Res<ExploderAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (exploder, current_fuse) = exploders.get_mut(trigger.target()).unwrap();

    if current_fuse {
        return;
    }

    let indicator = commands.spawn(indicator(&assets, &mut materials)).id();

    commands
        .entity(exploder)
        .insert((Fuse::new(trigger.event().0), Indicator(indicator)))
        .add_child(indicator);
}

fn tick_explosion(mut fuses: Query<&mut Fuse>, time: Res<Time>) {
    for mut fuse in &mut fuses {
        fuse.timer.tick(time.delta());
        if fuse.timer.just_finished() {
            fuse.countdown = fuse.countdown.saturating_sub(1);
        }
    }
}

fn animate_indicator(
    ignited_exploders: Query<(&Fuse, &Indicator)>,
    indicators: Query<&MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (fuse, indicator) in ignited_exploders {
        let indicator = indicators.get(indicator.0).unwrap();
        let material = materials.get_mut(indicator).unwrap();
        let color: Color = match fuse.countdown {
            3 => YELLOW.into(),
            2 => ORANGE.into(),
            _ => RED.into(),
        };
        material.base_color = color;
    }
}

#[derive(Event)]
pub struct HitByExplosion;

fn explode(
    mut commands: Commands,
    fuses: Query<(Entity, &Transform, &Fuse)>,

    spheres: Query<Has<Exploder>, With<Sphere>>,

    mut shake: Single<&mut Shake>,
    colliders: Query<&ColliderOf>,
    spatial_query: SpatialQuery,
) {
    let mut should_shake = false;
    for (entity, transform, fuse) in fuses {
        if fuse.countdown != 0 {
            continue;
        }

        let shape = Collider::sphere(EXPLOSION_RADIUS);
        let origin = transform.translation;
        let rotation = Quat::default();
        let filter = SpatialQueryFilter::from_mask(GameLayer::Sphere);
        let hits = spatial_query.shape_intersections(&shape, origin, rotation, &filter);

        for hit in hits {
            let Ok(collider) = colliders.get(hit) else {
                //means the collider has been despawned already.
                continue;
            };
            let body = collider.body;
            if body == entity {
                commands.entity(entity).trigger(DestroySphere);
                continue;
            }
            let Ok(is_exploder) = spheres.get(body) else {
                continue;
            };

            // if it's an exploder, it'll explode in 1 second. otherwise, lfg
            if is_exploder {
                commands.trigger_targets(LightFuse(1), body);
            } else {
                commands.entity(body).trigger(DestroySphere);
            }
        }
        should_shake = true;
    }
    if should_shake {
        shake.add_trauma(0.3);
    }
}
