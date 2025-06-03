use std::time::Duration;

use avian3d::prelude::Sensor;
use bevy::prelude::*;
use bevy_trauma_shake::Shake;

use crate::gameplay::{
    GameSet, GameState,
    sphere::{BeginDespawning, KeepOnCollideWith, SphereAssets, SphereType, sphere_defaults},
};

const EXPLOSION_RADIUS: f32 = 5.;

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
            explode
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
impl Default for Fuse {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
            countdown: 3,
        }
    }
}

fn start_despawn(
    trigger: Trigger<BeginDespawning>,
    mut commands: Commands,
    exploders: Query<Entity, With<Exploder>>,
) {
    let exploder = exploders.get(trigger.target()).unwrap();
    commands.entity(exploder).insert(Fuse::default());
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

fn explode(mut commands: Commands, fuses: Query<(Entity, &Fuse)>, mut shake: Single<&mut Shake>) {
    let mut should_shake = false;
    for (exploder, fuse) in fuses {
        if fuse.countdown != 0 {
            continue;
        }
        info!("boom.");
        should_shake = true;
        commands.entity(exploder).despawn()
    }
    if should_shake {
        shake.add_trauma(0.3);
    }
}
