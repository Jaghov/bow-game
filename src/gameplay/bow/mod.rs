use std::{collections::VecDeque, f32::consts::PI};

use bevy::prelude::*;
use pull::{PullStrength, Pulling};

use crate::{Screen, asset_tracking::LoadResource};

use super::{GameSet, cursor::CursorPosition};

mod animation;
pub mod pull;

const EPS: f32 = 1e-3;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<BowAssets>()
        .load_resource::<BowAssets>();

    app.add_plugins((pull::plugin, animation::plugin));

    app.add_systems(OnEnter(Screen::Gameplay), spawn_bow)
        .add_systems(
            Update,
            (update_bow_transform, update_bow_rotation_not_pulling).in_set(GameSet::Update),
        );
}
#[derive(Resource, Asset, Reflect, Clone)]
struct BowAssets {
    #[dependency]
    scene: Handle<Scene>,
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

#[derive(Component)]
#[require(PullStrength)]
pub struct Bow;

fn spawn_bow(mut commands: Commands, assets: Res<BowAssets>) {
    info!("Spawning bow");
    commands
        .spawn((Bow, SceneRoot(assets.scene.clone())))
        .observe(animation::setup_animations);
}

fn update_bow_transform(
    cursor: Res<CursorPosition>,
    mut bow: Query<&mut Transform, (With<Bow>, Without<Pulling>)>,
) {
    let Ok(mut bow) = bow.single_mut() else {
        return;
    };
    let Some(position) = cursor.current() else {
        return;
    };
    bow.translation = position;
}

fn update_bow_rotation_not_pulling(
    cursor: Res<CursorPosition>,
    mut bow: Query<&mut Transform, (With<Bow>, Without<Pulling>)>,
    mut last_positions: Local<VecDeque<Vec3>>,
    mut bow_should_rotation: Local<Quat>,
) {
    //number of positions to keep track of
    const NUM_POS_TO_TRACK: usize = 5;
    const CURSOR_POS_THRESHOLD: f32 = 3.;
    const ROTATION_SPEED: f32 = 0.15;

    let Some(position) = cursor.current() else {
        return;
    };
    /*
    if the number of positions is < 5, push regardless.
    if positions == 5, determine if
    */
    let mut adjust_should_rot = false;
    if last_positions.len() < NUM_POS_TO_TRACK {
        last_positions.push_back(position);
        adjust_should_rot = true;
    } else if last_positions.back().is_some_and(|lp| {
        (lp.x - position.x).abs() > CURSOR_POS_THRESHOLD
            || (lp.y - position.y).abs() > CURSOR_POS_THRESHOLD
    }) {
        last_positions.pop_front();
        last_positions.push_back(position);
        adjust_should_rot = true;
    }

    if adjust_should_rot && last_positions.len() >= 2 {
        let mut weighted_direction = Vec3::ZERO;
        let mut total_weight = 0.0;

        // Calculate weighted direction from consecutive position pairs
        for i in 1..last_positions.len() {
            let prev_pos = last_positions[i - 1];
            let curr_pos = last_positions[i];
            let direction = curr_pos - prev_pos;

            // Give more weight to recent movements
            let weight = i as f32;
            weighted_direction += direction * weight;
            total_weight += weight;
        }

        if total_weight > 0.0 && weighted_direction.length() > EPS {
            weighted_direction /= total_weight;
            let angle = weighted_direction.y.atan2(weighted_direction.x);
            *bow_should_rotation = Quat::from_rotation_z(angle + PI);
        }
    }

    let Ok(mut bow) = bow.single_mut() else {
        return;
    };

    // Smoothly interpolate to the target rotation
    bow.rotation = bow.rotation.slerp(*bow_should_rotation, ROTATION_SPEED);
}
