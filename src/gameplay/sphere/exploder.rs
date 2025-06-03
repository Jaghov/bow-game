use std::{f32::consts::FRAC_PI_2, time::Duration};

use avian3d::prelude::{Collider, Sensor, SpatialQuery, SpatialQueryFilter};
use bevy::{
    color::palettes::css::{ORANGE, RED, YELLOW},
    prelude::*,
};
use bevy_trauma_shake::Shake;

use crate::gameplay::{
    GameSet, GameState,
    sphere::{
        BeginDespawning, DespawnStarted, KeepOnCollideWith, Sphere, SphereAssets, SphereType,
        sphere_defaults,
    },
};

const EXPLOSION_RADIUS: f32 = 8.;

pub fn exploder(assets: &SphereAssets) -> impl Bundle {
    (
        sphere_defaults(assets),
        (
            Exploder,
            SphereType::Exploder,
            Sensor,
            MeshMaterial3d(assets.exploder.clone()),
        ),
    )
}

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
        let torus = meshes.add(Torus::new(EXPLOSION_RADIUS - 0.5, EXPLOSION_RADIUS));

        Self { torus }
    }
}

#[derive(Component)]
#[require(KeepOnCollideWith = KeepOnCollideWith::Sphere)]
pub struct Exploder;

fn insert_exploder(trigger: Trigger<OnAdd, Exploder>, mut commands: Commands) {
    info!("observed new normal insert");
    commands.entity(trigger.target()).observe(start_despawn);
}

#[derive(Component, Debug)]
struct Fuse {
    timer: Timer,
    countdown: usize,
}

impl Fuse {
    fn new(ticks: usize) -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
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
        Transform::from_rotation(Quat::from_rotation_x(FRAC_PI_2)),
    )
}

fn start_despawn(trigger: Trigger<BeginDespawning>, mut commands: Commands) {
    commands.trigger_targets(LightFuse(3), trigger.target());
}

fn light_fuse(
    trigger: Trigger<LightFuse>,
    mut commands: Commands,
    exploders: Query<Entity, With<Exploder>>,
    assets: Res<ExploderAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let exploder = exploders.get(trigger.target()).unwrap();

    let indicator = commands.spawn(indicator(&assets, &mut materials)).id();

    commands
        .entity(exploder)
        .insert((Fuse::new(trigger.event().0), Indicator(indicator)))
        .add_child(indicator);
}

fn tick_explosion(mut fuses: Query<&mut Fuse>, time: Res<Time>) {
    for mut fuse in &mut fuses {
        info!("fuse: {:?}", fuse);
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

fn explode(
    mut commands: Commands,
    fuses: Query<(&Transform, &Fuse)>,

    spheres: Query<Has<Exploder>, With<Sphere>>,

    mut shake: Single<&mut Shake>,
    spatial_query: SpatialQuery,
) {
    let mut should_shake = false;
    for (transform, fuse) in fuses {
        if fuse.countdown != 0 {
            continue;
        }

        info!("boom.");

        let shape = Collider::sphere(EXPLOSION_RADIUS);
        let origin = transform.translation;
        let rotation = Quat::default();
        let filter = SpatialQueryFilter::default();
        let hits = spatial_query.shape_intersections(&shape, origin, rotation, &filter);

        for hit in hits {
            let Ok(is_exploder) = spheres.get(hit) else {
                continue;
            };

            commands.entity(hit).insert(DespawnStarted);
            // if it's an exploder, it'll explode in 1 second. otherwise, lfg
            if is_exploder {
                commands.trigger_targets(LightFuse(2), hit);
            } else {
                commands.entity(hit).trigger(BeginDespawning);
            }
        }

        //let origin = transform.translation;

        should_shake = true;
        //commands.entity(exploder).despawn()
    }
    if should_shake {
        shake.add_trauma(0.3);
    }
}
