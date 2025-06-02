use avian3d::prelude::*;
use bevy::{platform::time::Instant, prelude::*};

use super::{Arrow, Canceled};

/// the max linear velocity speed of the arrow
pub const STRENGTH_MULT: f32 = 60.;

/// the arrow will be fired, but will be canceled if this velocity is not reached
const THRESHOLD: f32 = 15.;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(fire_arrow);
}

#[derive(Event)]
pub struct FireArrow(f32);

impl FireArrow {
    // takes in a value 0, 1
    pub fn new(pull_strength: f32) -> Self {
        Self(pull_strength.powi(2) * STRENGTH_MULT)
    }
}

#[derive(Component, Clone)]
#[require(CollisionEventsEnabled)]
pub struct Fired;

fn fire_arrow(
    trigger: Trigger<FireArrow>,
    mut commands: Commands,
    mut arrows: Query<(Entity, &Rotation, &mut LinearVelocity), (With<Arrow>, Without<Fired>)>,
) {
    let strength = trigger.event().0;
    for (arrow, rotation, mut lvel) in &mut arrows {
        let velocity = rotation.0 * Vec3::new(0., strength, 0.);
        lvel.0 = velocity;
        let mut arrow_commands = commands.entity(arrow);
        arrow_commands.insert(Fired);
        if strength < THRESHOLD {
            arrow_commands.insert((Canceled(Instant::now()), GravityScale(1.)));
        }
    }
}
