use std::{f32::consts::PI, path::Path};

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    gameplay::{ArrowSet, arrow::NockedOn, cursor::CursorPosition},
    rand::random_range,
    settings::Settings,
};

mod animation;
mod timefreeze;

mod primary;
pub use primary::*;

use super::arrow::{FireArrow, ReadyArrow};

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
        .add_systems(Update, update_pull_rotation.in_set(ArrowSet::UpdateBow))
        .add_observer(play_draw_on_ready_arrow)
        .add_observer(play_shot_on_fire_arrow);
}

#[derive(Component, Reflect)]
pub struct Bow;

#[derive(Resource, Asset, Reflect, Clone)]
pub struct BowAssets {
    #[dependency]
    pub scene: Handle<Scene>,
    #[dependency]
    pull_string: Handle<AnimationClip>,
    #[dependency]
    bow_draw: Handle<AudioSource>,
    #[dependency]
    bow_shoot: Handle<AudioSource>,
}
impl FromWorld for BowAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            scene: assets.load("models/BowFix.glb#Scene0"),
            pull_string: assets.load("models/BowFix.glb#Animation0"),
            bow_draw: assets.load(Path::new("audio/sfx/BowDrawSFX_V2.flac")),
            bow_shoot: assets.load(Path::new("audio/sfx/BowShootSFX_V2.flac")),
        }
    }
}

/// the max linear velocity speed of the arrow
const STRENGTH_MULT: f32 = 60.;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = NockedOn)]
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

    /// based on the current strength, this returns the
    /// velocity of the arrow
    pub fn arrow_velocity(&self) -> f32 {
        self.strength().powi(2) * STRENGTH_MULT
    }
}

fn play_draw_on_ready_arrow(
    _: Trigger<ReadyArrow>,
    assets: Res<BowAssets>,
    mut commands: Commands,
    settings: Res<Settings>,
) {
    commands.spawn((
        AudioPlayer::new(assets.bow_draw.clone()),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Once,
            speed: random_range(0.9..1.1), // Varied sfx speed to keep sounds more interesting
            volume: settings.sfx,
            ..Default::default()
        },
    ));
}

fn play_shot_on_fire_arrow(
    _: Trigger<FireArrow>,
    assets: Res<BowAssets>,
    mut commands: Commands,
    settings: Res<Settings>,
) {
    commands.spawn((
        AudioPlayer::new(assets.bow_shoot.clone()),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Once,
            speed: random_range(0.9..1.1),
            volume: settings.sfx,
            ..Default::default()
        },
    ));
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
