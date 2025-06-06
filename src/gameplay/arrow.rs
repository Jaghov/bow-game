use std::{f32::consts::FRAC_PI_2, time::Duration};

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    gameplay::{
        GameSet,
        bow::BowArrow,
        level::Walls,
        sphere::{FromMultiply, HitByExplosion, ShouldMultiply},
    },
    third_party::avian3d::GameLayer,
    world::GAME_PLANE,
};

use super::ArrowSet;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<NockedOn>()
        .register_type::<Arrow>()
        .register_type::<Canceled>()
        .register_type::<MaxFlightTime>()
        .register_type::<ArrowAssets>()
        .load_resource::<ArrowAssets>();

    app.add_systems(
        Update,
        update_unfired_arrow_transform.in_set(ArrowSet::UpdateArrow),
    )
    .add_systems(Update, tick_flight_time.in_set(GameSet::TickTimers))
    .add_systems(PostUpdate, (reset_flight_time, despawn_arrows).chain())
    .add_observer(spawn_arrow)
    .add_observer(add_arrow_colliders);
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct ArrowAssets {
    #[dependency]
    pub glowing: Handle<Scene>,
    #[dependency]
    pub normal: Handle<Scene>,
}
impl FromWorld for ArrowAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            glowing: assets.load("models/ArrowGlow.glb#Scene0"),
            normal: assets.load("models/Arrow.glb#Scene0"),
        }
    }
}

#[derive(Event)]
pub struct ReadyArrow(Entity);

impl ReadyArrow {
    pub fn for_bow(bow: Entity) -> Self {
        Self(bow)
    }
}

const ARROW_RADIUS: f32 = 0.1;
const ARROW_LEN: f32 = 3.5;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = BowArrow)]
pub struct NockedOn(Entity);

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(RigidBody = RigidBody::Dynamic)]
#[require(GravityScale = GravityScale(0.))]
#[require(LockedAxes = LockedAxes::ROTATION_LOCKED.lock_translation_z())]
#[require(Transform = Transform::default().with_scale(Vec3::splat(2.)))]
#[require(MaxFlightTime)]
pub struct Arrow {
    pub bounces: u8,
}

fn spawn_arrow(trigger: Trigger<ReadyArrow>, mut commands: Commands, assets: Res<ArrowAssets>) {
    info!("spawning arrow");
    commands
        .spawn((
            Name::new("Arrow"),
            Arrow::default(),
            SceneRoot(assets.glowing.clone()),
            NockedOn(trigger.event().0),
        ))
        .observe(fire_arrow)
        .observe(cancel_arrow);
}

// we will always overwrite children of arrow with 2 colliders
fn add_arrow_colliders(trigger: Trigger<OnAdd, Arrow>, mut commands: Commands) {
    let collider = Collider::capsule(ARROW_RADIUS, ARROW_LEN);

    let sensor = commands
        .spawn((
            collider.clone(),
            Sensor,
            CollisionLayers::new(
                GameLayer::ArrowSensor,
                [GameLayer::ArrowSensor, GameLayer::Sphere, GameLayer::Walls],
            ),
            CollisionEventsEnabled,
        ))
        .observe(wall_collision_flip)
        .id();

    let arrow_collider = commands
        .spawn((
            collider,
            ColliderDensity(10.),
            CollisionLayers::new(
                GameLayer::Arrow,
                [GameLayer::Arrow, GameLayer::Sphere, GameLayer::Backdrop],
            ),
        ))
        .id();

    commands
        .entity(trigger.target())
        .add_children(&[sensor, arrow_collider])
        .observe(despawn_on_explosion);
}

fn wall_collision_flip(
    trigger: Trigger<OnCollisionStart>,
    mut arrows: Query<(&mut Position, &mut Rotation), With<Arrow>>,
    everyone_else: Query<(&Position, &Rotation), Without<Arrow>>,
    walls: Query<(), With<Walls>>,
    colliders: Query<&ColliderOf>,
    positions: Query<&GlobalTransform>,
    collisions: Collisions,
) {
    let Ok((mut position, mut rotation)) = arrows.get(trigger.target()) else {
        return;
    };
    let Ok(collider) = colliders.get(trigger.collider) else {
        return;
    };

    if walls.get(collider.body).is_err() {
        return;
    };

    let Some(contact_pair) = collisions.get(trigger.target(), trigger.collider) else {
        info!("no contact pair!");
        return;
    };

    let Some(deepest_contact) = contact_pair.find_deepest_contact() else {
        warn!("multiplier was hit, but couldn't find deepest contact point!");
        return;
    };

    let local_point = if contact_pair.collider2 == trigger.collider {
        deepest_contact.local_point1
    } else {
        deepest_contact.local_point2
    };

    info!("Collided, local_point = {:?}", local_point);

    //todo
}

fn update_unfired_arrow_transform(
    mut arrows: Query<(&mut Transform, &NockedOn), Without<BowArrow>>,
    bow: Query<(&Transform, &BowArrow)>,
) {
    for (mut arrow, arrow_of) in &mut arrows {
        let Ok((bow, pull_strength)) = bow.get(arrow_of.0) else {
            continue;
        };
        // since the strength is from 0, 1, that scales from 0 to this number
        const BOW_RIGIDITY: f32 = 5.;
        /// this is how far to translate the arrow to sit on the bow string
        const STRING_OFFSET: f32 = -3.;
        let sv = pull_strength.strength() * BOW_RIGIDITY;
        let strength_vec = bow.rotation * Vec3::new(sv + STRING_OFFSET, 0., 0.);
        arrow.translation = bow.translation + strength_vec;
        let (z, _, _) = bow.rotation.to_euler(EulerRot::ZXY);
        arrow.rotation = Quat::from_rotation_z(z + FRAC_PI_2);
    }
}

/// the arrow will be fired, but will be canceled if this velocity is not reached
//TODO: set this back to 15ish
pub const ARROW_VELOCITY_THRESHOLD: f32 = 0.;

#[derive(Event)]
pub struct FireArrow;

fn fire_arrow(
    trigger: Trigger<FireArrow>,
    mut commands: Commands,
    mut arrows: Query<(&Rotation, &mut LinearVelocity, &NockedOn)>,
    mut pull_strength: Query<&BowArrow, Without<NockedOn>>,
) {
    info!("fire arrow event");
    let Ok((rotation, mut lvel, arrow_of)) = arrows.get_mut(trigger.target()) else {
        //clickr giht click too quickly;
        return;
    };

    let Ok(pull_strength) = pull_strength.get_mut(arrow_of.0) else {
        return;
    };

    let arrow_velocity = pull_strength.arrow_velocity();

    let velocity = rotation.0 * Vec3::new(0., arrow_velocity, 0.);
    lvel.0 = velocity;
    let mut arrow_commands = commands.entity(trigger.target());
    arrow_commands.remove::<NockedOn>();
    if arrow_velocity >= ARROW_VELOCITY_THRESHOLD {
        arrow_commands.observe(on_multiply);
    } else {
        commands.trigger_targets(CancelArrow, trigger.target());
    }
}
fn despawn_on_explosion(trigger: Trigger<HitByExplosion>, mut commands: Commands) {
    commands.entity(trigger.target()).try_despawn();
}

fn on_multiply(
    trigger: Trigger<ShouldMultiply>,
    mut commands: Commands,
    arrows: Query<(&Transform, &LinearVelocity, &SceneRoot), With<Arrow>>,
) {
    let event = trigger.event();
    let Ok((arrow_trn, lvel, scene_root)) = arrows.get(trigger.target()) else {
        warn!("Arrow was commanded to multiply, but its required components were not found!");
        return;
    };

    let multiply_origin = event.local_point.with_z(GAME_PLANE);

    for rotation_offset in &event.rot_offset {
        let quatrot = Quat::from_rotation_z(*rotation_offset);
        let rotation = arrow_trn.rotation * Quat::from_rotation_z(*rotation_offset);

        let velocity = quatrot * lvel.0;
        let offset = velocity.normalize_or_zero() * 4.;

        let transform = Transform::from_translation(multiply_origin + offset)
            .with_rotation(rotation)
            .with_scale(arrow_trn.scale);

        commands
            .spawn((
                Name::new("Cloned arrow"),
                Arrow::default(),
                transform,
                LinearVelocity(velocity),
                FromMultiply::default(),
                scene_root.clone(),
            ))
            .observe(on_multiply)
            .observe(despawn_on_explosion);
    }
}

#[derive(Event)]
pub struct CancelArrow;

#[derive(Component, Reflect)]
pub struct Canceled;

fn cancel_arrow(trigger: Trigger<CancelArrow>, mut commands: Commands) {
    info!("cancel arrow event");
    // this may have been done already if the firing speed is too low
    commands.entity(trigger.target()).try_remove::<NockedOn>();

    commands.entity(trigger.target()).insert((
        Canceled,
        // allows for z translation
        LockedAxes::new(),
        MaxFlightTime::new(Duration::from_secs(5)),
        GravityScale(1.),
    ));
}

// how long an arrow can fly without bouncing
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MaxFlightTime(Timer);
impl MaxFlightTime {
    pub fn new(duration: Duration) -> Self {
        Self(Timer::new(duration, TimerMode::Once))
    }
}

impl Default for MaxFlightTime {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs(10), TimerMode::Once))
    }
}

fn tick_flight_time(mut timers: Query<&mut MaxFlightTime, Without<NockedOn>>, time: Res<Time>) {
    for mut timer in &mut timers {
        timer.0.tick(time.delta());
    }
}

fn reset_flight_time(mut timers: Query<&mut MaxFlightTime, (Changed<Arrow>, Without<NockedOn>)>) {
    for mut timer in &mut timers {
        timer.0.reset();
    }
}
fn despawn_arrows(mut commands: Commands, timers: Query<(Entity, &MaxFlightTime)>) {
    for (entity, timer) in timers {
        if timer.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
