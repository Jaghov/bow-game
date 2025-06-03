use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    gameplay::{ArrowSet, arrow::ArrowOf, cursor::CursorPosition},
};

mod animation;
mod primary;
mod quiver;
mod timefreeze;
/// how far from the bow the player must draw bow
const MAX_RADIUS: f32 = 10.;
const EPS: f32 = 1e-3;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Bow>()
        .register_type::<BowArrow>()
        .register_type::<BowAssets>()
        .load_resource::<BowAssets>();

    app.add_plugins((timefreeze::plugin, animation::plugin, primary::plugin));

    app.add_systems(Update, update_pull_strength.in_set(ArrowSet::ProcessInput))
        .add_systems(Update, update_pull_rotation.in_set(ArrowSet::UpdateBow));
}

#[derive(Component, Reflect)]
pub struct Bow;

#[derive(Resource, Asset, Reflect, Clone)]
pub struct BowAssets {
    #[dependency]
    pub scene: Handle<Scene>,
    #[dependency]
    pull_string: Handle<AnimationClip>,
}
impl FromWorld for BowAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            scene: assets.load("models/BowFix.glb#Scene0"),
            pull_string: assets.load("models/BowFix.glb#Animation0"),
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = ArrowOf)]
pub struct BowArrow {
    #[relationship]
    arrow: Entity,
    pull_strength: f32,
}

impl BowArrow {
    pub fn arrow(&self) -> Entity {
        self.arrow
    }
    /// Returns a value from 0. to 1.
    pub fn strength(&self) -> f32 {
        self.pull_strength
    }
    /// clamps a set value from 0 to 1
    pub fn set_strength(&mut self, val: f32) {
        self.pull_strength = val.clamp(0., 1.)
    }
}

fn update_pull_strength(mut bow: Query<(&mut BowArrow, &Transform)>, cursor: Res<CursorPosition>) {
    let Some(cursor_position) = cursor.last() else {
        return;
    };
    let Ok((mut strength, transform)) = bow.single_mut() else {
        return;
    };
    let pull_start = transform.translation;
    let distance = (cursor_position - pull_start).length();

    // this will get clamped if the distance is greater than the max radius
    let pull_strength = distance / MAX_RADIUS;

    strength.set_strength(pull_strength);
}

fn update_pull_rotation(
    mut bow: Query<&mut Transform, (With<Bow>, With<BowArrow>)>,
    cursor: Res<CursorPosition>,
) {
    let Ok(mut bow) = bow.single_mut() else {
        return;
    };
    let Some(current_position) = cursor.current() else {
        return;
    };
    let direction = bow.translation - current_position;
    if direction.length_squared() < EPS {
        return;
    }
    let angle = direction.y.atan2(direction.x);
    bow.rotation = Quat::from_rotation_z(angle + PI);
}
