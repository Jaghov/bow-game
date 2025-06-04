use std::{f32::consts::FRAC_PI_2, time::Duration};

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    gameplay::{GameSet, bow::BowArrow, sphere::ShouldMultiply},
    world::GAME_PLANE,
};

use super::ArrowSet;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ArrowOf>()
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
    .add_observer(spawn_arrow);
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

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = BowArrow)]
pub struct ArrowOf(Entity);

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(RigidBody = RigidBody::Dynamic)]
#[require(Collider = Collider::capsule(0.1, 3.5))]
#[require(GravityScale = GravityScale(0.))]
#[require(LockedAxes = LockedAxes::new().lock_translation_z())]
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
            ArrowOf(trigger.event().0),
        ))
        .observe(fire_arrow)
        .observe(cancel_arrow);
}

fn update_unfired_arrow_transform(
    mut arrows: Query<(&mut Transform, &ArrowOf), Without<BowArrow>>,
    bow: Query<(&Transform, &BowArrow)>,
) {
    for (mut arrow, arrow_of) in &mut arrows {
        let Ok((bow, pull_strength)) = bow.get(arrow_of.0) else {
            continue;
        };
        // since the strength is from 0, 1, that scales from 0 to this number
        const BOW_RIGIDITY: f32 = 3.;
        /// this is how far to translate the arrow to sit on the bow string
        const STRING_OFFSET: f32 = -1.5;
        let sv = pull_strength.strength() * BOW_RIGIDITY;
        let strength_vec = bow.rotation * Vec3::new(sv + STRING_OFFSET, 0., 0.);
        arrow.translation = bow.translation + strength_vec;
        let (z, _, _) = bow.rotation.to_euler(EulerRot::ZXY);
        arrow.rotation = Quat::from_rotation_z(z + FRAC_PI_2);
    }

    //todo
}

/// the arrow will be fired, but will be canceled if this velocity is not reached
//TODO: set this back to 15ish
pub const ARROW_VELOCITY_THRESHOLD: f32 = 0.;

#[derive(Event)]
pub struct FireArrow;

// impl FireArrow {
//     // takes in a value 0, 1
//     pub fn new(pull_strength: f32) -> Self {
//         Self(pull_strength.powi(2) * STRENGTH_MULT)
//     }
// }

fn fire_arrow(
    trigger: Trigger<FireArrow>,
    mut commands: Commands,
    mut arrows: Query<(&Rotation, &mut LinearVelocity, &ArrowOf)>,
    mut pull_strength: Query<&BowArrow, Without<ArrowOf>>,
) {
    info!("fire arrow event");
    let (rotation, mut lvel, arrow_of) = arrows
        .get_mut(trigger.target())
        .expect("target to be an arrow");

    let Ok(pull_strength) = pull_strength.get_mut(arrow_of.0) else {
        return;
    };

    let arrow_velocity = pull_strength.arrow_velocity();

    let velocity = rotation.0 * Vec3::new(0., arrow_velocity, 0.);
    lvel.0 = velocity;
    let mut arrow_commands = commands.entity(trigger.target());
    arrow_commands.remove::<ArrowOf>();
    if arrow_velocity >= ARROW_VELOCITY_THRESHOLD {
        arrow_commands.observe(on_multiply);
    } else {
        commands.trigger_targets(CancelArrow, trigger.target());
    }
}

fn on_multiply(
    trigger: Trigger<ShouldMultiply>,
    mut commands: Commands,
    arrows: Query<(&Transform, &Collider, &LinearVelocity, &SceneRoot), With<Arrow>>,
) {
    let event = trigger.event();
    let Ok((arrow_trn, collider, lvel, scene_root)) = arrows.get(trigger.target()) else {
        warn!("Arrow was commanded to multiply, but its required components were not found!");
        return;
    };

    let multiply_origin = event.local_point.with_z(GAME_PLANE);

    for rotation_offset in &event.rot_offset {
        let quatrot = Quat::from_rotation_z(*rotation_offset);
        let rotation = arrow_trn.rotation * Quat::from_rotation_z(*rotation_offset);

        let velocity = quatrot * lvel.0;
        let offset = velocity.normalize() * 2.2;

        let transform = Transform::from_translation(multiply_origin + offset)
            .with_rotation(rotation)
            .with_scale(arrow_trn.scale);

        commands
            .spawn((
                Arrow::default(),
                transform,
                LinearVelocity(velocity),
                collider.clone(),
                scene_root.clone(),
            ))
            .observe(on_multiply);
    }
}

#[derive(Event)]
pub struct CancelArrow;

#[derive(Component, Reflect)]
pub struct Canceled;

fn cancel_arrow(trigger: Trigger<CancelArrow>, mut commands: Commands) {
    info!("cancel arrow event");
    // this may have been done already if the firing speed is too low
    commands.entity(trigger.target()).try_remove::<ArrowOf>();

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

fn tick_flight_time(mut timers: Query<&mut MaxFlightTime, Without<ArrowOf>>, time: Res<Time>) {
    for mut timer in &mut timers {
        timer.0.tick(time.delta());
    }
}

fn reset_flight_time(mut timers: Query<&mut MaxFlightTime, (Changed<Arrow>, Without<ArrowOf>)>) {
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
