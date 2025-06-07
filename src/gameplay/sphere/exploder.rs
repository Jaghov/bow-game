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
        sphere::{Absorber, DestroySphere, FromMultiply, Sphere, SphereAssets},
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

#[derive(Component, Default)]
#[require(Sphere)]
pub struct Exploder;

fn insert_exploder(
    trigger: Trigger<OnAdd, Exploder>,
    absorbers: Query<(), With<Absorber>>,
    mut commands: Commands,
    assets: Res<SphereAssets>,
) {
    let mut commands = commands.entity(trigger.target());
    if absorbers.get(trigger.target()).is_err() {
        commands.insert((
            CollisionLayers::new(
                GameLayer::Sphere,
                [GameLayer::ArrowSensor, GameLayer::Sphere],
            ),
            MeshMaterial3d(assets.exploder.clone()),
        ));
    }

    commands
        .observe(light_fuse_on_collision)
        .observe(light_fuse)
        .observe(|trigger: Trigger<HitByExplosion>, mut commands: Commands| {
            //light a much smaller fuse if hit by an explosion
            commands.trigger_targets(LightFuse(1), trigger.target());
        });
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
pub struct LightFuse(pub usize);

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
    colliders: Query<&ColliderOf>,
    mut commands: Commands,
) {
    let Ok(collider) = colliders.get(trigger.event().collider) else {
        return;
    };

    if ignore.get(collider.body).is_ok() {
        return;
    }
    commands.trigger_targets(LightFuse(3), trigger.target());
}

fn light_fuse(
    trigger: Trigger<LightFuse>,
    mut commands: Commands,
    mut exploders: Query<(Entity, Has<Fuse>, Has<Indicator>), With<Exploder>>,
    assets: Res<ExploderAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (exploder, current_fuse, has_indicator) = exploders.get_mut(trigger.target()).unwrap();

    if current_fuse {
        return;
    }

    if !has_indicator {
        let indicator = commands.spawn(indicator(&assets, &mut materials)).id();

        commands
            .entity(exploder)
            .insert(Indicator(indicator))
            .add_child(indicator);
    }

    commands
        .entity(exploder)
        .insert((Fuse::new(trigger.event().0)));
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
    ignited_exploders: Query<(Option<&Fuse>, &Indicator)>,
    indicators: Query<&MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (fuse, indicator) in ignited_exploders {
        let indicator = indicators.get(indicator.0).unwrap();
        let material = materials.get_mut(indicator).unwrap();
        let color: Color = match fuse {
            Some(fuse) => match fuse.countdown {
                3 => YELLOW.into(),
                2 => ORANGE.into(),
                _ => RED.into(),
            },
            None => Srgba::new(1., 1., 1., 0.2).into(),
        };
        material.base_color = color;
        if fuse.is_none() {
            material.alpha_mode = AlphaMode::Blend;
        }
    }
}

#[derive(Event)]
pub struct HitByExplosion {
    explosion_location: Vec2,
    exploder_was_from_multiply: bool,
}
impl HitByExplosion {
    fn new(explosion_location: Vec2, was_from_multiple: bool) -> Self {
        Self {
            explosion_location,
            exploder_was_from_multiply: was_from_multiple,
        }
    }
    pub fn location(&self) -> Vec2 {
        self.explosion_location
    }
    pub fn was_from_multiple(&self) -> bool {
        self.exploder_was_from_multiply
    }
}

fn explode(
    mut commands: Commands,
    fuses: Query<(Entity, &Transform, Has<FromMultiply>, &Fuse)>,
    mut shake: Single<&mut Shake>,
    colliders: Query<&ColliderOf>,
    spatial_query: SpatialQuery,
) {
    let mut should_shake = false;
    for (entity, transform, from_multiply, fuse) in fuses {
        if fuse.countdown != 0 {
            continue;
        }

        let shape = Collider::sphere(EXPLOSION_RADIUS);
        let origin = transform.translation;
        let rotation = Quat::default();
        let filter = SpatialQueryFilter::from_mask([GameLayer::Sphere, GameLayer::Arrow]);
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
            commands.trigger_targets(
                HitByExplosion::new(transform.translation.xy(), from_multiply),
                body,
            );
        }
        should_shake = true;
        commands.entity(entity).try_remove::<Fuse>();
    }
    if should_shake {
        shake.add_trauma(0.3);
    }
}
